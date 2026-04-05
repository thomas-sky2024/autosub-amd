<script lang="ts">
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import pkg from "diff-match-patch";
  const { diff_match_patch: DiffMatchPatch, DIFF_DELETE, DIFF_INSERT, DIFF_EQUAL } = pkg;
  import { jobStore, generateSRTContent } from "$lib/jobStore";
  import { onMount, onDestroy } from "svelte";

  // ── State ─────────────────────────────────────────────────────────────
  let textOriginal = $state(""); // Bản gốc (Text thuần)
  let textAI = $state("");       // Bản AI (Nội dung file SRT)
  let textResult = $state("");   // Kết quả SRT sau khi đồng bộ
  
  type Token = { type: number; text: string; };
  let diffTokens = $state<Token[]>([]);
  let activeView = $state<"diff" | "result">("result");
  let matches = $state({ total: 0, matched: 0, percent: 0 });

  // ── Auto-fill khi mở tab ──────────────────────────────────────────────
  onMount(() => {
    // Tự động generate SRT từ kết quả của AI nếu có
    if ($jobStore.segments && $jobStore.segments.length > 0) {
      textAI = generateSRTContent($jobStore.segments);
    }
  });

  // ── Actions: File & Utils ─────────────────────────────────────────────
  async function loadFile(target: "original" | "ai") {
    const filePath = await open({
      multiple: false,
      filters: [{ name: "Text/SRT Files", extensions: ["txt", "srt"] }],
    });
    
    if (filePath && typeof filePath === "string") {
      const content = await readTextFile(filePath);
      if (target === "original") textOriginal = content;
      else textAI = content;
    }
  }

  async function openPurpleCulture() {
    await openUrl("https://www.purpleculture.net/traditional-simplified-converter/");
  }

  function cleanCN(s: string) { 
    return (s || "").replace(/[^\u4e00-\u9fa5]/g, ""); 
  }

  // ── Logic 1: Diff-Match-Patch (Bỏ qua timestamp) ──────────────────────
  function runDiff() {
    const refText = cleanCN(textOriginal);
    const aiText = cleanCN(textAI);

    if (!refText || !aiText) {
      alert("Vui lòng cung cấp văn bản chứa ký tự tiếng Trung hợp lệ!");
      return;
    }

    const dmp = new DiffMatchPatch();
    // dmp.Diff_Timeout = 0; 
    const diffs = dmp.diff_main(aiText, refText); // Diff từ AI lên Gốc
    dmp.diff_cleanupSemantic(diffs);
    
    diffTokens = diffs.map(([type, text]) => ({ type, text }));
    
    let matched = 0;
    diffs.forEach(([op, txt]) => {
        if (op === 0) matched += txt.length;
    });

    matches = {
        total: refText.length,
        matched: matched,
        percent: refText.length > 0 ? (matched / refText.length) * 100 : 0
    };

    activeView = "diff";
  }

  // ── Logic 2: Đồng bộ thuật toán SRT ────────────────────
  function syncSRT() {
    // 1. Lọc toàn bộ chữ Hán từ bản gốc
    const refChars = Array.from(cleanCN(textOriginal));
    
    // 2. Chia block SRT của bản AI
    const blocks = textAI.trim().split(/\n\s*\n/);
    let ci = 0; // Con trỏ cho refChars

    const outBlocks = blocks.map(block => {
      const lines = block.split('\n');
      const tsIdx = lines.findIndex(l => l.includes('-->'));
      if (tsIdx === -1) return block; // Không phải subtitle block

      const subLines = lines.slice(tsIdx + 1);
      
      // Đếm chữ Hán trong block AI
      const aiSubText = subLines.join('');
      const aiCNCount = (aiSubText.match(/[\u4e00-\u9fa5]/g) || []).length;

      // Lấy đúng số lượng chữ Hán tương ứng từ refChars
      let newChars = '';
      for (let k = 0; k < aiCNCount && ci < refChars.length; k++, ci++) {
        newChars += refChars[ci];
      }

      // Thay thế chữ Hán, giữ nguyên dấu câu/khoảng trắng
      let nci = 0;
      const newSubLines = subLines.map(line => {
        let nl = '';
        for (const ch of line) {
          if (/[\u4e00-\u9fa5]/.test(ch)) {
            nl += nci < newChars.length ? newChars[nci++] : ch;
          } else {
            nl += ch; // Dấu câu, số, chữ la-tinh...
          }
        }
        return nl;
      });

      // Lắp ráp lại block (Index + Timestamp + NewLines)
      return [...lines.slice(0, tsIdx + 1), ...newSubLines].join('\n');
    });

    textResult = outBlocks.join('\n\n');
    activeView = "result";
  }

  // ── Xuất File ─────────────────────────────────────────────────────────
  async function downloadSRT() {
    if (!textResult) return;
    const path = await save({ filters: [{ name: "SRT", extensions: ["srt"] }], defaultPath: "synced_result.srt" });
    if (path) await writeTextFile(path, textResult);
  }

  function downloadTXT() {
    const blob = new Blob([cleanCN(textOriginal)], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "original_script.txt";
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="compare-layout h-full flex flex-col gap-5 p-6 overflow-hidden">
  
  <!-- Header Actions -->
  <div class="action-bar flex justify-between items-center bg-[#13141c]/50 backdrop-blur-md border border-[#2a2d3e] p-4 rounded-xl shadow-lg">
    <div class="flex gap-3">
        <button class="btn btn-outline flex items-center gap-2" onclick={openPurpleCulture}>
            <span class="icon">🌐</span> Web Giản/Phồn thể
        </button>
    </div>
    
    <div class="flex gap-3">
      <button class="btn btn-secondary flex items-center gap-2" onclick={runDiff} disabled={!textOriginal || !textAI}>
        <span class="icon">🔍</span> So Sánh Text
      </button>
      <button class="btn btn-primary flex items-center gap-2" onclick={syncSRT} disabled={!textOriginal || !textAI}>
        <span class="icon">✨</span> Đồng Bộ SRT
      </button>
    </div>
  </div>

  <!-- Input Grid -->
  <div class="grid grid-cols-2 gap-5 h-[30%] min-h-[220px]">
    <div class="input-panel col-original flex flex-col gap-2">
      <div class="panel-header flex justify-between items-center px-1">
        <span class="label-text">Kịch Bản Gốc (Tham chiếu)</span>
        <button class="btn-text" onclick={() => loadFile('original')}>Tải file txt...</button>
      </div>
      <div class="textarea-wrap flex-1">
        <textarea 
            class="custom-textarea" 
            bind:value={textOriginal} 
            placeholder="Dán text gốc vào đây để làm dữ liệu nguồn..."
        ></textarea>
      </div>
    </div>
    
    <div class="input-panel col-ai flex flex-col gap-2">
      <div class="panel-header flex justify-between items-center px-1">
        <span class="label-text">Văn Bản AI (SRT / Whisper)</span>
        <button class="btn-text" onclick={() => loadFile('ai')}>Tải file srt...</button>
      </div>
      <div class="textarea-wrap flex-1">
        <textarea 
            class="custom-textarea" 
            bind:value={textAI} 
            placeholder="Nội dung AI sẽ tự động hiện ở đây sau khi Transcribe xong..."
        ></textarea>
      </div>
    </div>
  </div>

  <!-- Results View -->
  <div class="results-container flex-1 flex flex-col min-h-0 bg-[#0e1016] border border-[#2a2d3e] rounded-xl overflow-hidden shadow-inner">
    <div class="results-header flex justify-between items-center p-4 border-b border-[#2a2d3e] bg-[#13141c]/40">
      <div class="flex items-center gap-4">
          <h3 class="results-title">
            {activeView === 'diff' ? 'Kết Quả So Sánh' : 'Kết Quả SRT Đồng Bộ'}
          </h3>
          {#if activeView === 'diff' && matches.total > 0}
            <div class="stats-badge">
                <span class="pct">{Math.round(matches.percent)}%</span>
                <span class="detail">{matches.matched}/{matches.total} chữ đúng</span>
            </div>
          {/if}
      </div>
      
      <div class="flex gap-2">
          {#if activeView === 'result' && textResult}
            <button class="btn btn-secondary btn-sm" onclick={downloadTXT}>⬇ Tải TXT</button>
            <button class="btn btn-primary btn-sm" onclick={downloadSRT}>⬇ Tải SRT</button>
          {/if}
      </div>
    </div>

    <div class="view-content flex-1 overflow-y-auto p-6 scroll-smooth">
      {#if activeView === 'diff'}
        <div class="diff-view font-serif text-xl leading-relaxed">
            {#if diffTokens.length > 0}
                {#each diffTokens as token}
                    {#if token.type === DIFF_EQUAL}
                        <span class="token-equal">{token.text}</span>
                    {:else if token.type === DIFF_INSERT}
                        <span class="token-insert" title="Có trong gốc, thiếu trong AI">{token.text}</span>
                    {:else if token.type === DIFF_DELETE}
                        <span class="token-delete" title="AI nghe nhầm/thừa">{token.text}</span>
                    {/if}
                {/each}
            {:else}
                <div class="empty-state">
                    Nhấn "So Sánh Text" để xem sự khác biệt giữa AI và Bản Gốc.
                </div>
            {/if}
        </div>
      {:else}
        {#if textResult}
          <pre class="srt-preview whitespace-pre-wrap font-mono text-sm leading-relaxed">{textResult}</pre>
        {:else}
          <div class="empty-state grayscale opacity-50">
            <div class="icon">✨</div>
            <p>Nhấn "Đồng Bộ SRT" để ghép chữ từ Bản Gốc vào Timestamp của Bản AI.</p>
            <p class="sub">Thuật toán sẽ tự động khớp từng ký tự tiếng Trung một cách chính xác.</p>
          </div>
        {/if}
      {/if}
    </div>
  </div>
</div>

<style>
  /* Layout & Panels */
  .compare-layout {
    font-family: 'Inter', -apple-system, sans-serif;
    animation: fadeIn 0.3s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .input-panel {
    min-width: 0;
  }

  .label-text {
    font-size: 0.75rem;
    font-weight: 700;
    color: #6b7194;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .btn-text {
    font-size: 0.7rem;
    color: #7c8cf8;
    background: none;
    border: none;
    cursor: pointer;
    font-weight: 600;
  }

  .btn-text:hover {
    text-decoration: underline;
  }

  /* Components */
  .custom-textarea {
    width: 100%;
    height: 100%;
    background: #1a1b28;
    border: 1px solid #2a2d3e;
    border-radius: 10px;
    padding: 1rem;
    color: #c4c8e2;
    font-size: 1rem;
    font-family: 'Songti SC', 'SimSun', serif;
    line-height: 1.6;
    resize: none;
    outline: none;
    transition: all 0.2s ease;
    box-shadow: inset 0 2px 4px rgba(0,0,0,0.2);
  }

  .custom-textarea:focus {
    border-color: #7c8cf8;
    background: #1e1f2f;
    box-shadow: 0 0 0 3px rgba(124, 140, 248, 0.15), inset 0 2px 4px rgba(0,0,0,0.2);
  }

  .btn {
    padding: 0.6rem 1.2rem;
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .btn-sm {
    padding: 0.4rem 0.8rem;
    font-size: 0.75rem;
  }

  .btn-primary {
    background: linear-gradient(135deg, #7c8cf8, #a78bfa);
    color: white;
    border: none;
    box-shadow: 0 4px 12px rgba(124, 140, 248, 0.3);
  }

  .btn-primary:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 6px 16px rgba(124, 140, 248, 0.4);
    filter: brightness(1.1);
  }

  .btn-secondary {
    background: #252840;
    color: #a5b4fc;
    border: 1px solid #3a3e5c;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #2d3254;
    border-color: #4c5277;
  }

  .btn-outline {
    background: transparent;
    border: 1px solid #2a2d3e;
    color: #c4c8e2;
  }

  .btn-outline:hover:not(:disabled) {
    background: rgba(255,255,255,0.05);
    border-color: #4b5563;
  }

  .btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    filter: grayscale(1);
  }

  /* Tokens & Diff View */
  .diff-view {
    color: #8b92b8;
  }

  .token-equal {
    opacity: 0.8;
  }

  .token-insert {
    background: rgba(16, 185, 129, 0.15);
    color: #34d399;
    border-bottom: 2px solid rgba(16, 185, 129, 0.3);
    padding: 0 2px;
    border-radius: 2px;
  }

  .token-delete {
    background: rgba(239, 68, 68, 0.15);
    color: #f87171;
    text-decoration: line-through;
    padding: 0 2px;
    border-radius: 2px;
    opacity: 0.6;
  }

  .results-title {
    font-size: 0.875rem;
    font-weight: 700;
    color: #94a3b8;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .stats-badge {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      background: rgba(124, 140, 248, 0.1);
      border: 1px solid rgba(124, 140, 248, 0.2);
      padding: 0.2rem 0.6rem;
      border-radius: 20px;
  }

  .stats-badge .pct {
      color: #7c8cf8;
      font-weight: 800;
      font-size: 0.75rem;
  }

  .stats-badge .detail {
      color: #64748b;
      font-size: 0.7rem;
      font-weight: 500;
  }

  .empty-state {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    color: #475569;
    gap: 0.5rem;
  }

  .empty-state .icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .empty-state p { font-size: 0.95rem; font-weight: 500; }
  .empty-state .sub { font-size: 0.8rem; opacity: 0.7; }

  .srt-preview {
    color: #e2e8f0;
  }

  /* Custom Scrollbar */
  ::-webkit-scrollbar { width: 8px; }
  ::-webkit-scrollbar-track { background: transparent; }
  ::-webkit-scrollbar-thumb { background: #2a2d3e; border-radius: 10px; }
  ::-webkit-scrollbar-thumb:hover { background: #3a3e5c; }
</style>
