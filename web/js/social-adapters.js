/**
 * DONK AI — Federated Social Memory Adapters (TikTok, Instagram, Farcaster, Bluesky)
 * Generates campaign deep links, canvas share cards, and platform-specific external references.
 */

class SocialMemoryAdapters {
  constructor() {
    this.baseUrl = window.location.origin || "https://1977.donkai.org";
  }

  // 1. Generate Platform Deep Link
  generateDeepLink(platform, topicId, mode = "blind", campaign = "do_not_search_it") {
    const params = new URLSearchParams({
      source: platform,
      campaign,
      prompt: topicId || "general_recall",
      mode
    });
    return `${this.baseUrl}/?${params.toString()}#remembranceWizard`;
  }

  // 2. Generate Shareable Memory Dilemma Card via HTML5 Canvas (1080x1920 Story format)
  async generateStoryCardCanvas(topic) {
    const canvas = document.createElement('canvas');
    canvas.width = 1080;
    canvas.height = 1920;
    const ctx = canvas.getContext('2d');

    // Background Dark Obsidian
    const bgGrad = ctx.createLinearGradient(0, 0, 0, 1920);
    bgGrad.addColorStop(0, '#07080c');
    bgGrad.addColorStop(0.5, '#0e101a');
    bgGrad.addColorStop(1, '#07080c');
    ctx.fillStyle = bgGrad;
    ctx.fillRect(0, 0, 1080, 1920);

    // Accent Glow Circles
    ctx.fillStyle = 'rgba(168, 85, 247, 0.12)';
    ctx.beginPath();
    ctx.arc(540, 400, 350, 0, Math.PI * 2);
    ctx.fill();

    // Top Header Badge
    ctx.fillStyle = '#a855f7';
    ctx.font = 'bold 36px "JetBrains Mono", monospace';
    ctx.textAlign = 'center';
    ctx.fillText('DONK AI  •  LPS-1 PROTOCOL', 540, 220);

    // Alert Callout
    ctx.fillStyle = '#fb7185';
    ctx.font = '900 54px "Inter", sans-serif';
    ctx.fillText('DO NOT SEARCH IT.', 540, 320);

    // Main Question
    ctx.fillStyle = '#ffffff';
    ctx.font = 'bold 64px "Inter", sans-serif';
    this.wrapText(ctx, topic.question, 540, 520, 880, 80);

    // Subtitle Callout
    ctx.fillStyle = '#94a3b8';
    ctx.font = '40px "Inter", sans-serif';
    ctx.fillText('What do you remember hearing or seeing?', 540, 880);
    ctx.fillText('Record before reading the comments.', 540, 940);

    // Option A Box
    ctx.fillStyle = '#131522';
    ctx.strokeStyle = '#a855f7';
    ctx.lineWidth = 4;
    ctx.roundRect(100, 1040, 880, 180, 16);
    ctx.fill();
    ctx.stroke();

    ctx.fillStyle = '#a855f7';
    ctx.font = 'bold 32px "JetBrains Mono", monospace';
    ctx.textAlign = 'left';
    ctx.fillText('OPTION A', 140, 1100);
    ctx.fillStyle = '#ffffff';
    ctx.font = 'bold 42px "Inter", sans-serif';
    ctx.fillText(topic.optionA.label, 140, 1165);

    // Option B Box
    ctx.fillStyle = '#131522';
    ctx.strokeStyle = '#38bdf8';
    ctx.lineWidth = 4;
    ctx.roundRect(100, 1260, 880, 180, 16);
    ctx.fill();
    ctx.stroke();

    ctx.fillStyle = '#38bdf8';
    ctx.font = 'bold 32px "JetBrains Mono", monospace';
    ctx.fillText('OPTION B', 140, 1320);
    ctx.fillStyle = '#ffffff';
    ctx.font = 'bold 42px "Inter", sans-serif';
    ctx.fillText(topic.optionB.label, 140, 1385);

    // Footer
    ctx.textAlign = 'center';
    ctx.fillStyle = '#cbd5e1';
    ctx.font = 'bold 40px "JetBrains Mono", monospace';
    ctx.fillText('Seal your memory at 1977.donkai.org', 540, 1650);

    ctx.fillStyle = '#64748b';
    ctx.font = '30px "Inter", sans-serif';
    ctx.fillText('Your memory is real human data. Popularity != Truth.', 540, 1720);

    return canvas.toDataURL('image/png');
  }

  wrapText(ctx, text, x, y, maxWidth, lineHeight) {
    const words = text.split(' ');
    let line = '';
    for (let n = 0; n < words.length; n++) {
      const testLine = line + words[n] + ' ';
      const metrics = ctx.measureText(testLine);
      const testWidth = metrics.width;
      if (testWidth > maxWidth && n > 0) {
        ctx.fillText(line, x, y);
        line = words[n] + ' ';
        y += lineHeight;
      } else {
        line = testLine;
      }
    }
    ctx.fillText(line, x, y);
  }
}

window.SocialMemoryAdapters = new SocialMemoryAdapters();
