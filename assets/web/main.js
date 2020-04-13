function loadConfig() {
    let urlParams = new URLSearchParams(window.location.search);

    if (
        !urlParams.has('font-faces')
        || !urlParams.has('font-sizes')
        || !urlParams.has('char-offset')
        || !urlParams.has('char-range')
        || !urlParams.has('file-name')
        || !urlParams.has('file-format')
    ) {
        return {"error": "invalid config"}
    }

    return {
        file: {
            name: urlParams.get('file-name'),
            format: urlParams.get('file-format'),
        },
        font: {
            faces: urlParams.get('font-faces').split(','),
            sizes: urlParams.get('font-sizes').split(','),
        },
        char: {
            offset: parseInt(urlParams.get('char-offset')),
            range: parseInt(urlParams.get('char-range')),
        }
    }
}

function generateDataSet(config) {
    let dataset = {
        config,
        data: {}
    };

    let paper = Raphael(0, 0, 0, 0);

    config.font.faces.forEach(function(face) {
        dataset.data[face] = {};

        config.font.sizes.forEach(function(size) {
            dataset.data[face][size] = {
                'boxes': draw(paper, face, size, config.char.offset, config.char.range)
            };
        });
    });

    paper.remove();

    return dataset;
}

function draw(paper, fontFace, fontSize, charOffset, charRange) {
    let boxes = [];

    for (let x=charOffset; x<=charRange; x++) {
        let str = paper.text(0, 0, String.fromCharCode(x));

        str.attr('font-family', fontFace);
        str.attr('font-size', fontSize);

        let boundingBox = str.getBBox();

        str.remove();

        boxes.push([boundingBox.width, boundingBox.height]);
    }

    return boxes;

}

window.onload = function() {
    let config = loadConfig();
    let data = null;

    if (config['error'] !== undefined) { data = config; }
    else { data = generateDataSet(config); }

    fetch(`/write/?filename=${encodeURIComponent(config.file.name)}&format=${config.file.format}`, {
        method: 'post',
        body: JSON.stringify(data),
        headers: new Headers({"Content-Type": "application/json"})
    }).then((response) => {
        return response.text();
    })
    .then((data) => {
        if (config['error'] === undefined) { console.log(`Server response: ${data}`) }
    });
};