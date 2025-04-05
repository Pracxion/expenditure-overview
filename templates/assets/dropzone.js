document.addEventListener('DOMContentLoaded', function() {
    const dropzone = document.querySelector('.dropzone-container');
    const fileInput = document.getElementById('file-upload');
    const placeholder = document.querySelector('.dropzone-placeholder p');
    
    if (!dropzone || !fileInput || !placeholder) {
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
    
    // Handle file selection
    fileInput.addEventListener('change', function() {
        if (fileInput.files && fileInput.files[0]) {
            placeholder.textContent = fileInput.files[0].name;
        }
    });
    
    // Handle file drop
    dropzone.addEventListener('drop', handleDrop, false);
    
    function handleDrop(e) {
        const dt = e.dataTransfer;
        const files = dt.files;
        
        if (files && files.length) {
            fileInput.files = files;
            placeholder.textContent = files[0].name;
        }
    }
}); 