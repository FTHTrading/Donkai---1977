// donkai.org — interactive front-desk logic
// * animated boot terminal (matches donkai-node output)
// * two-zone museum with 10 seed relics
// * format war arena with pull-payment split animation
// * mint form with in-browser LPS-1 Merkle root + raw CIDv1 preview
//   (byte-identical to donkai-lps1 and donkai-ipfs Rust output)

const DonkaiApp = (() => {

  // ---------- Museum seed data (10 entries across 2 zones) ----------
  const MUSEUM = {
    1: [
      { year: '1975',      title: 'Pet Rock',
        stubbornness: 88,
        blurb: 'Boxed pebble with 32-page care manual. $3.95 retail. Established zero-utility PFP long before JPGs did.' },
      { year: '1983',      title: 'Cabbage Patch Kids',
        stubbornness: 74,
        blurb: 'Randomized traits + stamped adoption certificates + limited supply. Retail riots as proof-of-metadata scarcity.' },
      { year: '1993–1999', title: 'Beanie Babies',
        stubbornness: 82,
        blurb: '$60B secondary market at peak. Divorcing couples ordered by a judge to divide plush collateral on a Las Vegas courtroom floor.' },
      { year: '1996',      title: 'Tickle Me Elmo',
        stubbornness: 69,
        blurb: 'Retail brawls documented on national news. Peak of physical-supply-constrained dopamine distribution.' },
      { year: '2006',      title: 'Horse Armor DLC',
        stubbornness: 91,
        blurb: '$2.50 cosmetic microtransaction for Oblivion. Gamers loudly protested and privately purchased in record volume.' }
    ],
    2: [
      { year: '1976–1988', title: 'Betamax vs. VHS',
        stubbornness: 77,
        blurb: 'Beta had superior picture quality. VHS won on 2-hour runtime and open licensing. Convenience beat specs.' },
      { year: '1984–2015', title: 'Mac vs. Wintel',
        stubbornness: 65,
        blurb: 'Apple integrated hardware and software; Microsoft licensed the OS to every OEM. Open distribution captured 90%+ share for 30 years.' },
      { year: '1995–2008', title: 'Firewire vs. USB',
        stubbornness: 43,
        blurb: 'Firewire had better throughput. USB was cheaper to license and universal on every board. Standards win by ubiquity.' },
      { year: '2006–2008', title: 'HD-DVD vs. Blu-ray',
        stubbornness: 71,
        blurb: 'Sony bundled Blu-ray into every PlayStation 3. Warner Bros. defection ended the war in 18 months.' },
      { year: '2007–2015', title: 'iOS vs. Android',
        stubbornness: 58,
        blurb: "Apple's integrated stack captured margin; Android's OEM licensing captured volume. Two winners, one battlefield." }
    ]
  };

  // ---------- Format war matchups ----------
  const MATCHUPS = [
    { titleA: 'Betamax',  titleB: 'VHS',      historical: 'VHS (1976–1988)',       splitA: 38, splitB: 62 },
    { titleA: 'Mac',      titleB: 'Wintel',   historical: 'Wintel (1984–2015)',    splitA: 15, splitB: 85 },
    { titleA: 'HD-DVD',   titleB: 'Blu-ray',  historical: 'Blu-ray (2006–2008)',   splitA: 22, splitB: 78 }
  ];

  // ---------- Boot terminal animation ----------
  const BOOT_LINES = [
    '<span class="out-header">========================================================================</span>',
    '<span class="out-header"> DONKAI NETWORK  |  Chain ID 1977  |  Proof-of-Stubbornness + AFT 2/3</span>',
    '<span class="out-header">========================================================================</span>',
    '',
    '<span class="out-lps1">[LPS-1]</span>        root       = 0xd44862b67f8cd402079c9622cc46e3bc88a40e11c7b2577296dc7a78d041c478',
    '<span class="out-lps1">[LPS-1]</span>        leaves     = 3',
    '<span class="out-lps1">[LPS-1]</span>        audit      = 58 / 58 checks passed  (all_passed = true)',
    '<span class="out-lps1">[LPS-1]</span>        proof(#1)  = path_len=2 verified=true',
    '',
    '<span class="out-ipfs">[IPFS]</span>         raw CIDv1     = bafkreid56oyvmj6koan3uehq4z3hrzb743mf5bvcmkieya4elnkz35gtoe',
    '<span class="out-ipfs">[IPFS]</span>         dag-pb CIDv1  = bafybeihwotj5puutrrcsnbspx6uiaallnbqvuanjg53tswkajuihxh35ry',
    '<span class="out-ipfs">[IPFS]</span>         Kubo endpoint = http://127.0.0.1:5001/api/v0',
    '',
    '<span class="out-pqc">[PQC]</span>          ml-dsa-87    pk_len=2592 sig_len=4627 verified=true',
    '',
    '<span class="out-consensus">[CONSENSUS]</span>    node       = donkai1val_ultra_asinine_01',
    '<span class="out-consensus">[CONSENSUS]</span>    weight     = 500000000000000000000000000000',
    '<span class="out-consensus">[CONSENSUS]</span>    2/3 BFT    = true',
    '',
    '<span class="out-policy">[POLICYGUARD]</span>  tier=D4Degenerate action=MINT_DONK_USD -> APPROVED  (2-of-2 required)'
  ];

  function animateTerminal() {
    const el = document.getElementById('boot-terminal');
    if (!el) return;
    let i = 0;
    const iv = setInterval(() => {
      if (i >= BOOT_LINES.length) {
        el.innerHTML = BOOT_LINES.join('\n') + '\n<span class="cursor"></span>';
        clearInterval(iv);
        return;
      }
      el.innerHTML = BOOT_LINES.slice(0, i + 1).join('\n') + '\n<span class="cursor"></span>';
      i++;
    }, 130);
  }

  // ---------- Museum render + tab switch ----------
  function renderMuseum(zone) {
    const grid = document.getElementById('museum-grid');
    if (!grid) return;
    grid.innerHTML = MUSEUM[zone].map(r => `
      <article class="relic">
        <div class="relic-year">${r.year}</div>
        <h4>${escapeHtml(r.title)}</h4>
        <p>${escapeHtml(r.blurb)}</p>
        <div class="relic-meta">
          <span>zone_${zone}</span>
          <span class="score">stubbornness · ${r.stubbornness}/100</span>
        </div>
      </article>
    `).join('');
  }

  function bindMuseumTabs() {
    document.querySelectorAll('.museum-tabs .tab').forEach(tab => {
      tab.addEventListener('click', () => {
        const z = parseInt(tab.dataset.zone, 10);
        document.querySelectorAll('.museum-tabs .tab').forEach(t => t.classList.toggle('active', t === tab));
        renderMuseum(z);
      });
    });
  }

  // ---------- Format war arena ----------
  function renderArena() {
    const grid = document.getElementById('arena-grid');
    if (!grid) return;
    grid.innerHTML = MATCHUPS.map((m, i) => `
      <article class="matchup" data-idx="${i}">
        <div class="matchup-header">Historical winner: ${escapeHtml(m.historical)}</div>
        <div class="matchup-title"><strong>${escapeHtml(m.titleA)}</strong> vs. <strong>${escapeHtml(m.titleB)}</strong></div>
        <div class="split-bar">
          <div class="split-a" style="flex:${m.splitA}">${escapeHtml(m.titleA)} · ${m.splitA}%</div>
          <div class="split-b" style="flex:${m.splitB}">${m.splitB}% · ${escapeHtml(m.titleB)}</div>
        </div>
        <div class="matchup-btns">
          <button data-side="A">Stake on ${escapeHtml(m.titleA)}</button>
          <button data-side="B">Stake on ${escapeHtml(m.titleB)}</button>
        </div>
      </article>
    `).join('');
    // Bind buttons after render (no inline onclick)
    grid.querySelectorAll('.matchup').forEach(card => {
      const idx = parseInt(card.dataset.idx, 10);
      card.querySelectorAll('.matchup-btns button').forEach(btn => {
        btn.addEventListener('click', () => stake(idx, btn.dataset.side));
      });
    });
  }

  function stake(idx, side) {
    const m = MATCHUPS[idx];
    // Shift 3 points toward chosen side, clamp opposite at 1
    if (side === 'A') { m.splitA += 3; m.splitB = Math.max(1, m.splitB - 3); }
    else               { m.splitB += 3; m.splitA = Math.max(1, m.splitA - 3); }
    const total = m.splitA + m.splitB;
    m.splitA = Math.round((m.splitA / total) * 100);
    m.splitB = 100 - m.splitA;
    renderArena();
  }

  // ---------- LPS-1 binary Merkle tree (matches donkai-lps1 Rust impl) ----------
  async function sha256Bytes(bytes) {
    const digest = await crypto.subtle.digest('SHA-256', bytes);
    return new Uint8Array(digest);
  }
  function toHex(bytes) {
    return Array.from(bytes).map(b => b.toString(16).padStart(2, '0')).join('');
  }

  async function computeLps1Root(text) {
    const paragraphs = text.split('\n\n').map(s => s.trim()).filter(Boolean);
    if (paragraphs.length === 0) {
      return '0x' + '0'.repeat(64);
    }
    let level = await Promise.all(
      paragraphs.map(p => sha256Bytes(new TextEncoder().encode(p)))
    );
    while (level.length > 1) {
      const next = [];
      for (let i = 0; i < level.length; i += 2) {
        const l = level[i];
        const r = (i + 1 < level.length) ? level[i + 1] : level[i]; // odd-tail duplication
        const combined = new Uint8Array(64);
        combined.set(l, 0);
        combined.set(r, 32);
        next.push(await sha256Bytes(combined));
      }
      level = next;
    }
    return '0x' + toHex(level[0]);
  }

  // ---------- Raw CIDv1 (codec 0x55, sha2-256) — matches donkai-ipfs ----------
  const B32_ALPHABET = 'abcdefghijklmnopqrstuvwxyz234567';
  function base32LowerNoPad(bytes) {
    let bits = 0, value = 0, out = '';
    for (const b of bytes) {
      value = (value << 8) | b;
      bits += 8;
      while (bits >= 5) {
        bits -= 5;
        out += B32_ALPHABET[(value >>> bits) & 0x1f];
      }
    }
    if (bits > 0) {
      out += B32_ALPHABET[(value << (5 - bits)) & 0x1f];
    }
    return out;
  }

  async function computeRawCidv1(text) {
    const digest = await sha256Bytes(new TextEncoder().encode(text));
    const cid = new Uint8Array(4 + 32);
    cid[0] = 0x01; // version 1
    cid[1] = 0x55; // codec raw
    cid[2] = 0x12; // multihash sha2-256
    cid[3] = 0x20; // digest length 32
    cid.set(digest, 4);
    return 'b' + base32LowerNoPad(cid); // 'b' = base32 lowercase multibase
  }

  // ---------- Mint form ----------
  function bindMintForm() {
    const form = document.getElementById('mint-form');
    if (!form) return;
    form.addEventListener('submit', async (ev) => {
      ev.preventDefault();
      const story = form.story.value;
      const [root, cid] = await Promise.all([
        computeLps1Root(story),
        computeRawCidv1(story)
      ]);
      document.getElementById('mint-root').value = root;
      document.getElementById('mint-ipfs').value = cid;
      renderCertificate({
        title:    (form.title.value || '').trim(),
        eraLabel: form.era.options[form.era.selectedIndex]?.text || '—',
        root,
        cid
      });
    });
    const reset = document.getElementById('mint-reset');
    if (reset) {
      reset.addEventListener('click', () => {
        form.reset();
        document.getElementById('mint-root').value = '';
        document.getElementById('mint-ipfs').value = '';
        const cert = document.getElementById('ownership-cert');
        if (cert) cert.hidden = true;
      });
    }
  }

  // ---------- ownership certificate ----------
  function renderCertificate({ title, eraLabel, root, cid }) {
    const cert = document.getElementById('ownership-cert');
    if (!cert) return;
    document.getElementById('cert-title').textContent = title || '—';
    document.getElementById('cert-era').textContent   = eraLabel || '—';
    document.getElementById('cert-root').textContent  = root;
    document.getElementById('cert-cid').textContent   = cid;
    document.getElementById('cert-time').textContent  = new Date().toISOString();
    cert.hidden = false;
    cert.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
  }

  // ---------- utils ----------
  function escapeHtml(s) {
    return String(s).replace(/[&<>"']/g, c => ({
      '&':'&amp;', '<':'&lt;', '>':'&gt;', '"':'&quot;', "'":'&#39;'
    })[c]);
  }

  // ---------- init ----------
  document.addEventListener('DOMContentLoaded', () => {
    animateTerminal();
    renderArena();
    renderMuseum(1);
    bindMuseumTabs();
    bindMintForm();
  });

  return {};
})();
