let identCount = 0;

const identCounter = (filename) => {
    console.log(`Visiting: ${filename}`);
    return {
        visitor: {
            Identifier() {
                console.log(identCount += 1)
            }
        }
    }
};

module.exports = identCounter;