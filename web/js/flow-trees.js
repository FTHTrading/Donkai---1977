/**
 * DONK AI — Interactive Flow Trees & Merkle Visualizer Engine
 */

class FlowTreeRenderer {
  /**
   * Renders an interactive LPS-1 Merkle tree onto an SVG container.
   */
  static renderMerkleTree(svgContainerId, leaves, activeLeafIndex = null) {
    const svg = document.getElementById(svgContainerId);
    if (!svg) return;
    svg.innerHTML = '';

    const width = svg.clientWidth || 800;
    const height = 340;
    svg.setAttribute('viewBox', `0 0 ${width} ${height}`);

    const nodeWidth = 130;
    const nodeHeight = 44;

    // Build layers
    let layers = [];
    let currentLayer = leaves.map((l, i) => ({
      id: `leaf-${i}`,
      label: l.label || `Leaf #${i}`,
      hash: l.hash,
      type: l.type || 'LEAF',
      x: 0,
      y: 0,
      isTarget: i === activeLeafIndex
    }));
    layers.push(currentLayer);

    while (currentLayer.len > 1 || layers.length < 3) {
      if (currentLayer.length === 1) break;
      let nextLayer = [];
      for (let i = 0; i < currentLayer.length; i += 2) {
        const left = currentLayer[i];
        const right = (i + 1 < currentLayer.length) ? currentLayer[i + 1] : left;
        nextLayer.push({
          id: `node-${layers.length}-${i / 2}`,
          label: (layers.length === 2) ? 'BUNDLE ROOT' : `NODE #${i / 2}`,
          hash: '0x' + (Math.random().toString(16) + '000000000000').slice(2, 10) + '...',
          type: (layers.length === 2) ? 'ROOT' : 'NODE',
          x: 0,
          y: 0,
          leftId: left.id,
          rightId: right.id
        });
      }
      layers.push(nextLayer);
      currentLayer = nextLayer;
    }

    // Assign positions
    const layerCount = layers.length;
    layers.forEach((layer, layerIdx) => {
      const y = height - 50 - (layerIdx * 110);
      const totalNodes = layer.length;
      const spacing = width / (totalNodes + 1);
      layer.forEach((node, nodeIdx) => {
        node.x = spacing * (nodeIdx + 1);
        node.y = y;
      });
    });

    // Draw connecting lines
    for (let l = 1; l < layers.length; l++) {
      const parentLayer = layers[l];
      const childLayer = layers[l - 1];
      parentLayer.forEach(parent => {
        const leftChild = childLayer.find(c => c.id === parent.leftId);
        const rightChild = childLayer.find(c => c.id === parent.rightId);
        if (leftChild) FlowTreeRenderer.drawLine(svg, leftChild.x, leftChild.y - nodeHeight/2, parent.x, parent.y + nodeHeight/2);
        if (rightChild && rightChild !== leftChild) FlowTreeRenderer.drawLine(svg, rightChild.x, rightChild.y - nodeHeight/2, parent.x, parent.y + nodeHeight/2);
      });
    }

    // Draw nodes
    layers.forEach(layer => {
      layer.forEach(node => {
        FlowTreeRenderer.drawNode(svg, node, nodeWidth, nodeHeight);
      });
    });
  }

  static drawLine(svg, x1, y1, x2, y2) {
    const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
    line.setAttribute('x1', x1);
    line.setAttribute('y1', y1);
    line.setAttribute('x2', x2);
    line.setAttribute('y2', y2);
    line.setAttribute('stroke', 'rgba(196, 203, 216, 0.3)');
    line.setAttribute('stroke-width', '2');
    line.setAttribute('stroke-dasharray', '4 2');
    svg.appendChild(line);
  }

  static drawNode(svg, node, w, h) {
    const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
    g.style.cursor = 'pointer';
    g.onclick = () => {
      alert(`LPS-1 Node Details:\nType: ${node.type}\nLabel: ${node.label}\nCommitment Hash: ${node.hash}`);
    };

    const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
    rect.setAttribute('x', node.x - w / 2);
    rect.setAttribute('y', node.y - h / 2);
    rect.setAttribute('width', w);
    rect.setAttribute('height', h);
    rect.setAttribute('rx', '6');

    if (node.type === 'ROOT') {
      rect.setAttribute('fill', '#1c2230');
      rect.setAttribute('stroke', '#7ca3d8');
      rect.setAttribute('stroke-width', '2');
    } else if (node.isTarget) {
      rect.setAttribute('fill', '#172b22');
      rect.setAttribute('stroke', '#7cd8a8');
      rect.setAttribute('stroke-width', '2');
    } else {
      rect.setAttribute('fill', '#13141c');
      rect.setAttribute('stroke', 'rgba(255,255,255,0.12)');
      rect.setAttribute('stroke-width', '1');
    }

    const title = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    title.setAttribute('x', node.x);
    title.setAttribute('y', node.y - 4);
    title.setAttribute('text-anchor', 'middle');
    title.setAttribute('fill', '#f1f5f9');
    title.setAttribute('font-size', '11');
    title.setAttribute('font-weight', '600');
    title.setAttribute('font-family', 'Inter, sans-serif');
    title.textContent = node.label;

    const hashText = document.createElementNS('http://www.w3.org/2000/svg', 'text');
    hashText.setAttribute('x', node.x);
    hashText.setAttribute('y', node.y + 12);
    hashText.setAttribute('text-anchor', 'middle');
    hashText.setAttribute('fill', '#94a3b8');
    hashText.setAttribute('font-size', '9');
    hashText.setAttribute('font-family', 'JetBrains Mono, monospace');
    hashText.textContent = node.hash ? node.hash.slice(0, 10) + '...' : '0x...';

    g.appendChild(rect);
    g.appendChild(title);
    g.appendChild(hashText);
    svg.appendChild(g);
  }
}

window.FlowTreeRenderer = FlowTreeRenderer;
