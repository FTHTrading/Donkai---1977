/**
 * DONK AI — Cinematic Archival Walkthrough & Visual Primitives
 * Powers the 5-stage interactive memory journey, audio toggle, particle canvas, and modal disclosures.
 */

class MemoryWalkthrough {
  constructor() {
    this.currentStage = 1;
    this.totalStages = 5;
    this.isAudioPlaying = false;
    this.audioCtx = null;
    this.initParticles();
    this.bindEvents();
  }

  initParticles() {
    const canvas = document.getElementById('ambientCanvas');
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    let width = canvas.width = window.innerWidth;
    let height = canvas.height = window.innerHeight;

    window.addEventListener('resize', () => {
      width = canvas.width = window.innerWidth;
      height = canvas.height = window.innerHeight;
    });

    const particles = [];
    for (let i = 0; i < 45; i++) {
      particles.push({
        x: Math.random() * width,
        y: Math.random() * height,
        radius: Math.random() * 1.8 + 0.6,
        vx: (Math.random() - 0.5) * 0.35,
        vy: -Math.random() * 0.45 - 0.1,
        color: i % 3 === 0 ? 'rgba(157, 126, 255, ' : (i % 3 === 1 ? 'rgba(255, 62, 98, ' : 'rgba(0, 229, 255, '),
        alpha: Math.random() * 0.6 + 0.2
      });
    }

    function animate() {
      ctx.clearRect(0, 0, width, height);
      for (const p of particles) {
        p.x += p.vx;
        p.y += p.vy;
        if (p.y < -10) { p.y = height + 10; p.x = Math.random() * width; }
        if (p.x < -10) p.x = width + 10;
        if (p.x > width + 10) p.x = -10;

        ctx.beginPath();
        ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
        ctx.fillStyle = p.color + p.alpha + ')';
        ctx.shadowBlur = 8;
        ctx.shadowColor = p.color + '0.8)';
        ctx.fill();
      }
      requestAnimationFrame(animate);
    }
    animate();
  }

  bindEvents() {
    // Stage navigation
    document.querySelectorAll('[data-goto-stage]').forEach(btn => {
      btn.addEventListener('click', (e) => {
        const target = parseInt(btn.getAttribute('data-goto-stage'), 10);
        this.goToStage(target);
      });
    });

    // Sound toggle
    const soundBtn = document.getElementById('soundToggleBtn');
    if (soundBtn) {
      soundBtn.addEventListener('click', () => this.toggleAmbientSound(soundBtn));
    }
  }

  goToStage(stageNum) {
    if (stageNum < 1 || stageNum > this.totalStages) return;
    this.currentStage = stageNum;

    document.querySelectorAll('.stage-scene').forEach(scene => {
      scene.classList.remove('active');
    });
    const targetScene = document.getElementById(`stageScene${stageNum}`);
    if (targetScene) {
      targetScene.classList.add('active');
      targetScene.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }

    document.querySelectorAll('.stage-dot').forEach((dot, idx) => {
      if (idx + 1 === stageNum) dot.classList.add('active');
      else if (idx + 1 < stageNum) dot.classList.add('completed');
      else { dot.classList.remove('active'); dot.classList.remove('completed'); }
    });
  }

  toggleAmbientSound(btn) {
    if (!this.audioCtx) {
      const AudioContext = window.AudioContext || window.webkitAudioContext;
      this.audioCtx = new AudioContext();
    }

    if (!this.isAudioPlaying) {
      this.audioCtx.resume();
      this.play528HzChime();
      this.isAudioPlaying = true;
      btn.innerHTML = '<span>🔊 528Hz Ambient On</span>';
      btn.classList.add('active');
    } else {
      this.audioCtx.suspend();
      this.isAudioPlaying = false;
      btn.innerHTML = '<span>🔇 Ambient Sound</span>';
      btn.classList.remove('active');
    }
  }

  play528HzChime() {
    if (!this.audioCtx) return;
    const osc = this.audioCtx.createOscillator();
    const gain = this.audioCtx.createGain();
    
    // 528Hz Transformation Frequency with subtle harmonic
    osc.type = 'sine';
    osc.frequency.setValueAtTime(528, this.audioCtx.currentTime);
    
    gain.gain.setValueAtTime(0.01, this.audioCtx.currentTime);
    gain.gain.exponentialRampToValueAtTime(0.08, this.audioCtx.currentTime + 1.5);
    
    osc.connect(gain);
    gain.connect(this.audioCtx.destination);
    osc.start();
  }
}

// Global initialization
window.addEventListener('DOMContentLoaded', () => {
  window.walkthrough = new MemoryWalkthrough();
});
