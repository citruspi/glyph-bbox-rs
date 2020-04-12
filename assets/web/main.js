function loadConfig() {
    let urlParams = new URLSearchParams(window.location.search);

    if (!urlParams.has('font-faces') || !urlParams.has('font-sizes')) {
        return {"error": "invalid config"}
    }

    return {
        font: {
            faces: urlParams.get('font-faces').split(','),
            sizes: urlParams.get('font-sizes').split(','),
        }
    }
}