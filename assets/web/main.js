function loadConfig() {
    let urlParams = new URLSearchParams(window.location.search);

    if (
        !urlParams.has('font-faces')
        || !urlParams.has('font-sizes')
        || !urlParams.has('char-offset')
        || !urlParams.has('char-range')
    ) {
        return {"error": "invalid config"}
    }

    return {
        font: {
            faces: urlParams.get('font-faces').split(','),
            sizes: urlParams.get('font-sizes').split(','),
        },
        char: {
            offset: urlParams.get('char-offset'),
            range: urlParams.get('char-range'),
        }
    }
}

window.onload = function() {
    let config = loadConfig();

    if (config['error'] !== undefined) { console.log(config); return }

    console.log(config);
};