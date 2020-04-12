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

    if (config['error'] !== undefined) { console.log(config); return }

    let dataset = generateDataSet(config);

    console.log(dataset);
};