// liquid-scene.js — Three.js hero background: chrome icosahedron morphing under neon lights.
// Loaded as ES module. Uses jsdelivr CDN import map defined in index.html.

import * as THREE from 'three';
import { RoomEnvironment } from 'three/addons/environments/RoomEnvironment.js';

const canvas = document.getElementById('hero-canvas');

// ---------- graceful degradation ----------
function hasWebGL() {
  try {
    const c = document.createElement('canvas');
    return !!(window.WebGLRenderingContext && (c.getContext('webgl2') || c.getContext('webgl')));
  } catch (_) { return false; }
}

const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
const smallScreen   = window.matchMedia('(max-width: 640px)').matches;

if (!canvas || !hasWebGL() || reducedMotion) {
  // Leave the CSS liquid-drift fallback in place and exit quietly.
  if (canvas) canvas.style.display = 'none';
} else {
  document.body.classList.add('webgl-on');
  boot(canvas);
}

// ---------- main ----------
function boot(canvas) {
  const renderer = new THREE.WebGLRenderer({
    canvas,
    antialias: !smallScreen,
    alpha: true,
    powerPreference: smallScreen ? 'low-power' : 'high-performance',
  });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, smallScreen ? 1.25 : 2));
  const parent = canvas.parentElement;
  const sizeToParent = () => {
    const w = parent.clientWidth, h = parent.clientHeight;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  };
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 1.15;

  const scene = new THREE.Scene();

  // ---- environment: chrome needs something to reflect ----
  const pmrem = new THREE.PMREMGenerator(renderer);
  scene.environment = pmrem.fromScene(new RoomEnvironment(), 0.04).texture;

  // ---- camera ----
  const camera = new THREE.PerspectiveCamera(38, 1, 0.1, 100);
  camera.position.set(0, 0, smallScreen ? 5.5 : 4.6);

  // ---- liquid metal blob (morphing chrome icosahedron) ----
  const detail = smallScreen ? 32 : 96;
  const geometry = new THREE.IcosahedronGeometry(1.55, Math.log2(detail) | 0);
  const material = new THREE.MeshPhysicalMaterial({
    color:              0xffffff,
    metalness:          1.0,
    roughness:          0.08,
    iridescence:        1.0,
    iridescenceIOR:     1.9,
    iridescenceThicknessRange: [100, 800],
    clearcoat:          1.0,
    clearcoatRoughness: 0.06,
    envMapIntensity:    1.4,
  });
  const blob = new THREE.Mesh(geometry, material);
  scene.add(blob);

  // Snapshot the base positions for morph displacement
  const positionAttr = geometry.attributes.position;
  const basePositions = new Float32Array(positionAttr.array);

  // ---- metal-tinted point lights: iceblue / rosegold / champagne / platinum ----
  // Just enough warm/cool bias to activate iridescence without introducing neon color.
  const l1 = new THREE.PointLight(0xc8d4e0, 22, 12, 2); l1.position.set(3.2,  2.2,  4.0);
  const l2 = new THREE.PointLight(0xd4b8b0, 20, 12, 2); l2.position.set(-3.0, -2.4, 3.6);
  const l3 = new THREE.PointLight(0xe0d8c8, 18, 10, 2); l3.position.set(0.5,  3.8, -3.0);
  const l4 = new THREE.PointLight(0xd8d8de, 16,  9, 2); l4.position.set(-1.5, 3.0, -1.5);
  scene.add(l1, l2, l3, l4);

  // ---- pointer parallax ----
  const pointer = { x: 0, y: 0, targetX: 0, targetY: 0 };
  window.addEventListener('pointermove', (e) => {
    const r = canvas.getBoundingClientRect();
    if (e.clientY < r.top || e.clientY > r.bottom) return;
    pointer.targetX = (e.clientX / window.innerWidth  - 0.5) * 2;
    pointer.targetY = (e.clientY / window.innerHeight - 0.5) * 2;
  }, { passive: true });

  // ---- deterministic pseudo-noise (no dep) ----
  // Layered sine/cos gives cheap smooth "liquid" displacement.
  function morphNoise(x, y, z, t) {
    const a = Math.sin(x * 1.7 + t * 0.9) * Math.cos(y * 2.1 - t * 0.7);
    const b = Math.sin(y * 1.3 - t * 1.1) * Math.cos(z * 1.9 + t * 0.6);
    const c = Math.sin(z * 1.5 + t * 0.8) * Math.cos(x * 2.3 - t * 0.5);
    return (a + b + c) / 3;
  }

  // ---- animation loop ----
  const AMPLITUDE = 0.18;
  let running = true;
  document.addEventListener('visibilitychange', () => { running = !document.hidden; if (running) tick(); });

  function tick(nowMs) {
    if (!running) return;
    const t = (nowMs || performance.now()) * 0.00035;

    // vertex displacement along the outward normal
    for (let i = 0; i < positionAttr.count; i++) {
      const ix = i * 3;
      const bx = basePositions[ix], by = basePositions[ix + 1], bz = basePositions[ix + 2];
      const len = Math.hypot(bx, by, bz);
      const n = morphNoise(bx, by, bz, t);
      const scale = 1 + (n * AMPLITUDE) / Math.max(len, 0.0001);
      positionAttr.array[ix]     = bx * scale;
      positionAttr.array[ix + 1] = by * scale;
      positionAttr.array[ix + 2] = bz * scale;
    }
    positionAttr.needsUpdate = true;
    geometry.computeVertexNormals();

    // smooth rotation + parallax lerp
    pointer.x += (pointer.targetX - pointer.x) * 0.03;
    pointer.y += (pointer.targetY - pointer.y) * 0.03;
    blob.rotation.x = t * 0.18 + pointer.y * 0.35;
    blob.rotation.y = t * 0.24 + pointer.x * 0.5;

    // subtly orbit lights
    l1.position.x = Math.sin(t * 0.4) * 3.4;
    l1.position.z = Math.cos(t * 0.4) * 3.4;
    l2.position.x = Math.sin(t * 0.5 + Math.PI) * 3.2;
    l2.position.z = Math.cos(t * 0.5 + Math.PI) * 3.2;

    renderer.render(scene, camera);
    requestAnimationFrame(tick);
  }

  sizeToParent();
  window.addEventListener('resize', sizeToParent);
  requestAnimationFrame(tick);
}
