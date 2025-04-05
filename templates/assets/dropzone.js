document.addEventListener('DOMContentLoaded', function () {
    const dropzone = document.querySelector('.dropzone-container');
    const fileInput = document.getElementById('file-upload');
    const placeholder = document.querySelector('.dropzone-placeholder p');
    const fileListContainer = document.querySelector('.file-list-container');

    if (!dropzone || !fileInput || !placeholder || !fileListContainer) {
        console.error('Dropzone elements not found');
        return;
    }

    // Handle drag events
    ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
        dropzone.addEventListener(eventName, preventDefaults, false);
    });

    function preventDefaults(e) {
        e.preventDefault();
        e.stopPropagation();
    }

    // Handle visual feedback
    ['dragenter', 'dragover'].forEach(eventName => {
        dropzone.addEventListener(eventName, highlight, false);
    });

    ['dragleave', 'drop'].forEach(eventName => {
        dropzone.addEventListener(eventName, unhighlight, false);
    });

    function highlight() {
        dropzone.classList.add('drag-over');
    }

    function unhighlight() {
        dropzone.classList.remove('drag-over');
    }

    function createFileList(files) {
        const dataTransfer = new DataTransfer();
        files.forEach(file => dataTransfer.items.add(file));
        return dataTransfer.files;
    }

    function removeFile(index) {
        const filesArray = Array.from(fileInput.files);
        filesArray.splice(index, 1);
        fileInput.files = createFileList(filesArray);
        updatePlaceholder(fileInput.files);
    }


    function updatePlaceholder(files) {
        if (!files || files.length === 0) {
            placeholder.textContent = 'Drop your CSV files here or click to browse';
            fileListContainer.innerHTML = '';
            return;
        }

        placeholder.textContent = files.length === 1
            ? '1 file selected'
            : `${files.length} files selected`;

        fileListContainer.innerHTML = '';

        for (let i = 0; i < files.length; i++) {
            const file = files[i];

            const fileChip = document.createElement('div');
            fileChip.className = 'file-chip';
            fileChip.title = file.name; // Show full name on hover

            const textSpan = document.createElement('span');
            let displayName = file.name;
            if (displayName.length > 30) {
                const extension = displayName.split('.').pop();
                displayName = displayName.substring(0, 27) + '...' + (extension ? '.' + extension : '');
            }
            textSpan.innerHTML = displayName

            const deleteBtn = document.createElement('span');
            deleteBtn.className = 'delete-chip';
            deleteBtn.innerHTML = '&times;';
            deleteBtn.title = 'Remove file';
            deleteBtn.dataset.index = i;

            deleteBtn.addEventListener('click', function (e) {
                e.preventDefault();
                e.stopPropagation();
                // Remove this specific file
                const index = parseInt(this.dataset.index, 10);
                removeFile(index);
            });

            fileChip.appendChild(textSpan);
            fileChip.appendChild(deleteBtn);
            fileListContainer.appendChild(fileChip);
        }
    }

    fileInput.addEventListener('change', function () {
        if (fileInput.files && fileInput.files.length > 0) {
            updatePlaceholder(fileInput.files);
        }
    });

    dropzone.addEventListener('drop', handleDrop, false);

    function handleDrop(e) {
        const dt = e.dataTransfer;
        const files = dt.files;

        if (files && files.length > 0) {
            fileInput.files = files;
            updatePlaceholder(files);
        }
    }
}); 