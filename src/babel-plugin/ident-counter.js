let identCount = 0;

const random_id = String(Math.floor(Math.random()*100));

const identCounter = () => {
    const renamed = new Set();
    return {
        visitor: {
            Identifier(nodePath) {
                if(renamed.has(nodePath.node.name)) return;
                nodePath.node.name = `${nodePath.node.name}_${random_id}_${identCount}`
                renamed.add(nodePath.node.name);
                identCount += 1;
            }
        }
    }
};

module.exports = identCounter;