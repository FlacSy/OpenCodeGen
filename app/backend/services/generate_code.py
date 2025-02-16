import ast
import re
import json
import zipfile
import code_generator
from enum import Enum
from pydantic import BaseModel
from difflib import get_close_matches


class Language(str, Enum):
    PYTHON = "python"
    PY = "py"
    TYPESCRIPT = "typescript"
    TS = "ts"
    RUST = "rust"
    RS = "rs"
    JAVA = "java"

class GeneratedCode(BaseModel):
    code: str

class OpenAPI(BaseModel):
    openapi: str
    language: Language

    def validate_openapi(self):
        try:
            json.loads(self.openapi)
        except:
            raise InputSyntaxError(f"Invalid OpenAPI JSON")
        return self

class InputCodeForArchive(BaseModel):
    code: str
    openapi: str
    language: Language

    def validate_language(self):
        python_language_options = ['py', 'python']
        if self.language.value not in python_language_options:
            raise InputSyntaxError("Invalid language, supports only Python")

class InputSyntaxError(Exception):
    def __init__(self, *args: object) -> None:
        super().__init__(*args)

class ClassVisitor(ast.NodeVisitor):
    def __init__(self):
        self.classes = {}

    def visit_ClassDef(self, node: ast.ClassDef):
        self.classes[node.name] = node
        self.generic_visit(node)

class CodeGenerator:
    def generate(self, openapi: OpenAPI) -> GeneratedCode:
        openapi_str = openapi.openapi
        language = openapi.language.value

        try:
            generated_code: str = code_generator.generate_code(openapi_str, language)  # type: ignore
        except Exception as e:
            raise ValueError(f"Error generating code: {e}")

        return GeneratedCode(code=generated_code)


    def _sanitize_import_name(self, name: str) -> str:
        return re.sub(r'(?<!^)(?=[A-Z])', '_', name).lower()

    def _group_classes_by_similarity(self, class_names, threshold=0.6):
        groups = {}
        for cls in class_names:
            match = get_close_matches(cls, groups.keys(), n=1, cutoff=threshold)
            if match:
                groups[match[0]].append(cls)
            else:
                groups[cls] = [cls]
        return groups

    def _extract_dependencies_for_class(self, cls_node: ast.AST):
        dependencies = set()
        for subnode in ast.walk(cls_node):
            if isinstance(subnode, ast.Name):
                if subnode.id and subnode.id[0].isupper():
                    dependencies.add(subnode.id)
            elif isinstance(subnode, ast.Constant) and isinstance(subnode.value, str):
                for match in re.findall(r'\b([A-Z][A-Za-z0-9_]*)\b', subnode.value):
                    dependencies.add(match)
        return dependencies

    def _generate_imports_for_group(self, group, class_to_file, all_classes):
        imports = set()
        typing_imports = set()
        builtin_types = {"int", "str", "bool", "float", "dict", "list", "tuple", "None"}
        
        for cls in group:
            node = all_classes[cls]
            deps = self._extract_dependencies_for_class(node)
            for dep in deps:
                if dep in group or dep in builtin_types:
                    continue
                if dep in {"List", "Union", "Optional", "Any"}:
                    typing_imports.add(dep)
                    continue
                if dep in class_to_file:
                    dep_file = class_to_file[dep]
                    if dep_file == class_to_file[cls]:
                        continue
                    imports.add(f"from .{dep_file} import {dep}")
        
        if typing_imports:
            imports.add("from typing import " + ", ".join(sorted(typing_imports)))

        imports.add("from pydantic import BaseModel, Field")
        return sorted(imports)

    def split_file_archive(self, code: str, output_archive: str):
        tree = ast.parse(code)
        visitor = ClassVisitor()
        visitor.visit(tree)
        all_classes = visitor.classes

        groups = self._group_classes_by_similarity(list(all_classes.keys()))

        class_to_file = {}
        group_file_names = {}
        for rep, group in groups.items():
            file_name = self._sanitize_import_name(group[0])
            group_file_names[rep] = file_name
            for cls in group:
                class_to_file[cls] = file_name

        with zipfile.ZipFile(output_archive, 'w', zipfile.ZIP_DEFLATED) as archive:
            for rep, group in groups.items():
                file_name = f"{group_file_names[rep]}.py"
                imports = self._generate_imports_for_group(group, class_to_file, all_classes)
                content = "\n".join(imports) + "\n\n"
                for cls in group:
                    class_code = ast.unparse(all_classes[cls])
                    content += class_code + "\n\n"
                
                archive.writestr(file_name, content)

        return output_archive