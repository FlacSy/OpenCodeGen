new Vue({
  el: '#app',
  data: {
    openapiInput: '',
    generatedCode: '',
    inputEditor: null,
    outputEditor: null,
    selectedLanguage: 'python',
    copyMessageVisible: false,
    errorMessageVisible: false,
  },
  mounted() {
    this.inputEditor = CodeMirror.fromTextArea(document.getElementById('openapi-input'), {
      lineNumbers: true,
      mode: 'application/json',
      theme: 'dracula',
      indentUnit: 2,
    });

    this.outputEditor = CodeMirror.fromTextArea(document.getElementById('generated-code-output'), {
      lineNumbers: true,
      mode: this.getModeForLanguage(this.selectedLanguage),
      theme: 'dracula',
      readOnly: true,
      indentUnit: 2,
      viewportMargin: Infinity,
    });

    const dropArea = document.getElementById("drop-zone");
    dropArea.addEventListener("dragover", this.highlight);
    dropArea.addEventListener("dragleave", this.unhighlight);
    dropArea.addEventListener("drop", this.handleDrop);
  },
  methods: {
    async generateCode() {
      try {
        const input = this.inputEditor.getValue();
        if (input.trim() === '') {
          this.showErrorMessage();
          return;
        }

        const jsonRegex = /^[\],:{}\s]*$/;
        if (!jsonRegex.test(input.replace(/\\["\\\/bfnrtu]/g, '@').replace(/"[^"\\\n\r]*"|true|false|null|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?/g, ']')
            .replace(/(?:^|:|,)(?:\s*\[)+/g, ''))) {
          this.errorMessage = 'Неверный формат JSON';
          this.showErrorMessage();
          return;
        }
        
        const response = await axios.post('/generate', {
          openapi: input,
          language: this.selectedLanguage,
        });

        console.log('Ответ от сервера:', response.data);

        if (response.data && response.data.body && response.data.body.generated_code) {
          this.generatedCode = response.data.body.generated_code;
          this.outputEditor.setValue(this.generatedCode);
          if (this.selectedLanguage === 'python' || this.selectedLanguage === 'py') {
            const zipButton = document.getElementById('generateZip');
            if (!zipButton) {
                console.log('No zip button');
                return;
            }
            zipButton.style.display = "flex";
        }        
        } else {
          this.errorMessage = 'Ошибка: Нет сгенерированного кода в ответе';
          this.showErrorMessage();
        }
      } catch (error) {
        console.error('Ошибка при запросе:', error);
        this.showErrorMessage();
      }
    },
    async generateZip() {
      try {
        const input = this.inputEditor.getValue();
        if (input.trim() === '') {
          this.showErrorMessage();
          return;
        }
        
        if (this.selectedLanguage !== 'python' && this.selectedLanguage !== 'py') {
          this.showErrorMessage();
          console.log(this.selectedLanguage);
          return;
        }
    
        const jsonRegex = /^[\],:{}\s]*$/;
        if (!jsonRegex.test(input.replace(/\\["\\\/bfnrtu]/g, '@').replace(/"[^"\\\n\r]*"|true|false|null|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?/g, ']')
          .replace(/(?:^|:|,)(?:\s*\[)+/g, ''))) {
          this.errorMessage = 'Неверный формат JSON';
          this.showErrorMessage();
          return;
        }
    
        try {
          const response = await axios.post('/zip-file', {
            openapi: this.inputEditor.getValue(),
            language: 'python',
            code: this.outputEditor.getValue(),
          }, {
            headers: { 'Content-Type': 'application/json' },
            responseType: 'blob',
          });

          if (response.status === 200) {
            const zipBlob = response.data;

            const link = document.createElement('a');
            link.href = URL.createObjectURL(zipBlob);
            link.download = 'generated.zip';
            link.click();
            
            console.log('ZIP-файл успешно загружен');
          }
        } catch (error) {
          console.error('Ошибка при запросе:', error.response ? error.response.data : error.message);
        }
    
      } catch (error) {
        console.error('Ошибка при запросе:', error);
        this.showErrorMessage();
      }
    },
    showErrorMessage() {
      this.errorMessageVisible = true;
      setTimeout(() => {
        this.errorMessageVisible = false;
      }, 3000);
    },
    getModeForLanguage(language) {
      switch (language) {
        case 'python':
          return 'python';
        case 'typescript':
          return 'javascript';
        case 'rust':
          return 'clike';
        case 'java':
          return 'clike';
        default:
          return 'text/plain';
      }
    },
    highlight(event) {
      event.preventDefault();
      event.stopPropagation();
      event.currentTarget.classList.add("highlight");
    },
    unhighlight(event) {
      event.preventDefault();
      event.stopPropagation();
      event.currentTarget.classList.remove("highlight");
    },
    handleDrop(event) {
      event.preventDefault();
      event.stopPropagation();
      this.unhighlight(event);

      const file = event.dataTransfer.files[0];
      if (!file) return;

      const allowedExtensions = ["json"]; // ["json", "yaml", "yml"]
      const fileExtension = file.name.split(".").pop().toLowerCase();

      if (!allowedExtensions.includes(fileExtension)) {
        this.errorMessage = "Поддерживаются только файлы .json";
        this.showErrorMessage();
        return;
      }

      const reader = new FileReader();
      reader.onload = (e) => {
        this.inputEditor.setValue(e.target.result);
        this.errorMessage = "";
      };

      reader.readAsText(file);
    },
    copyToClipboard() {
      const outputText = this.outputEditor.getValue();
      if (outputText) {
        navigator.clipboard.writeText(outputText)
          .then(() => {
            console.log("Код скопирован в буфер обмена!");
            this.showCopyMessage();
          })
          .catch(err => {
            console.error("Не удалось скопировать код: ", err);
          });
      } else {
        console.error("Поле для сгенерированного кода пустое!");
      }
    },
    showCopyMessage() {
      this.copyMessageVisible = true;
      setTimeout(() => {
        this.copyMessageVisible = false;
      }, 3000);
    }
  }
});
