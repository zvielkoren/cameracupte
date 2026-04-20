let invoke, listen;

let deviceInputEl, startBtnEl, previewImgEl, previewWrapperEl;
let createDeviceBtn, isoSelect, apertureSelect, statusPill, statusText;
const toastContainer = document.getElementById('toast-container');

// Debug overlay has been removed for production.

const _recentToasts = new Map();
function showToast(msg, type = 'success') {
  // Deduplicate: suppress identical messages within 3 seconds
  const key = `${type}:${msg}`;
  if (_recentToasts.has(key)) return;
  _recentToasts.set(key, true);
  setTimeout(() => _recentToasts.delete(key), 3000);

  const toast = document.createElement('div');
  toast.className = `toast ${type}`;
  
  // Choose icon based on type
  const icon = type === 'error' 
    ? `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-error"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg>`
    : `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-success"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>`;
    
  toast.innerHTML = `${icon} <span>${msg}</span>`;
  toastContainer.appendChild(toast);

  // Auto remove after 4 seconds
  setTimeout(() => {
    toast.classList.add('fade-out');
    setTimeout(() => toast.remove(), 300);
  }, 4000);
}

function updateStatus(isOnline) {
  if (isOnline) {
    statusPill.classList.remove('offline');
    statusPill.classList.add('online');
    statusText.textContent = 'Live';
    previewWrapperEl.classList.add('active');
  } else {
    statusPill.classList.remove('online');
    statusPill.classList.add('offline');
    statusText.textContent = 'Disconnected';
    previewWrapperEl.classList.remove('active');
    previewImgEl.style.opacity = 0;
  }
}

let isStreaming = false;

async function toggleStream(e) {
  if (e) e.preventDefault();
  
  if (isStreaming) {
    // Stop Stream
    try {
      await invoke("stopcamera");
      showToast("Stream stopped");
      startBtnEl.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="5 3 19 12 5 21 5 3"></polygon></svg> Start Stream`;
      updateStatus(false);
      isStreaming = false;
      startBtnEl.disabled = false;
    } catch (error) {
      showToast(`Stop failed: ${error}`, 'error');
    }
    return;
  }

  // Start Stream
  const devicePath = deviceInputEl.value;
  startBtnEl.disabled = true;
  startBtnEl.innerHTML = `<svg class="spinner" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="2" x2="12" y2="6"></line><line x1="12" y1="18" x2="12" y2="22"></line><line x1="4.93" y1="4.93" x2="7.76" y2="7.76"></line><line x1="16.24" y1="16.24" x2="19.07" y2="19.07"></line><line x1="2" y1="12" x2="6" y2="12"></line><line x1="18" y1="12" x2="22" y2="12"></line><line x1="4.93" y1="19.07" x2="7.76" y2="16.24"></line><line x1="16.24" y1="7.76" x2="19.07" y2="4.93"></line></svg> Connecting...`;

  try {
    const res = await invoke("startcamera", { devicePath });
    showToast("Camera stream active");
    startBtnEl.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect></svg> Stop Stream`;
    updateStatus(true);
    isStreaming = true;
    startBtnEl.disabled = false;
  } catch (error) {
    showToast(error, 'error');
    startBtnEl.disabled = false;
    startBtnEl.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="5 3 19 12 5 21 5 3"></polygon></svg> Start Stream`;
    updateStatus(false);
  }
}

async function createDevice() {
  createDeviceBtn.disabled = true;
  createDeviceBtn.classList.add('loading');
  try {
    const path = await invoke("createvirtualdevice");
    deviceInputEl.value = path;
    showToast(`Device ready at ${path}`);
  } catch (error) {
    showToast(`Initialization failed. See terminal.`, 'error');
  } finally {
    createDeviceBtn.disabled = false;
    createDeviceBtn.classList.remove('loading');
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  console.log("[APP] DOM Loaded. Initializing UI...");
  
  try {
    invoke = window.__TAURI__.core.invoke;
    listen = window.__TAURI__.event.listen;
    deviceInputEl = document.querySelector("#device-path");
    startBtnEl = document.querySelector("#start-btn");
    previewImgEl = document.querySelector("#preview-img");
    previewWrapperEl = document.querySelector("#preview-wrapper");
    createDeviceBtn = document.querySelector("#create-device-btn");
    isoSelect = document.querySelector("#iso-select");
    apertureSelect = document.querySelector("#aperture-select");
    statusPill = document.querySelector("#status-pill");
    statusText = document.querySelector(".status-text");

    if (!startBtnEl || !deviceInputEl) {
        throw new Error("Critical UI elements missing from DOM");
    }

    startBtnEl.addEventListener("click", (e) => {
        console.log("[IPC] Start/Stop Stream clicked");
        toggleStream(e);
    });
    
    if (createDeviceBtn) {
        createDeviceBtn.addEventListener("click", () => {
            console.log("[IPC] Create Device clicked");
            createDevice();
        });
    }


  let manualObjPctX = 0.5;
  let manualObjPctY = 0.5;

  previewWrapperEl.addEventListener("click", (e) => {
    if (!isStreaming) return;
    const rect = previewWrapperEl.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    manualObjPctX = x / rect.width;
    manualObjPctY = y / rect.height;
    
    const focusBox = document.querySelector("#focus-box");
    focusBox.style.left = `${x - 30}px`;
    focusBox.style.top = `${y - 30}px`;
    focusBox.style.display = 'block';
    
    // Trigger AF animation
    focusBox.classList.remove("locked");
    focusBox.classList.add("focusing");
    
    invoke("setzoomposition", { 
        x: Math.floor(manualObjPctX * 5184), 
        y: Math.floor(manualObjPctY * 3456) 
    }).then(() => {
        return invoke("triggerautofocus");
    }).then(() => {
        focusBox.classList.remove("focusing");
        focusBox.classList.add("locked");
        setTimeout(() => focusBox.classList.remove("locked"), 1500);
    }).catch(err => {
        focusBox.classList.remove("focusing");
        showToast(`AF Error: ${err}`, 'error');
    });
  });

  isoSelect.addEventListener("change", async (e) => {
    console.log("[IPC] ISO Change:", e.target.value);
    try { 
      await invoke("setiso", { iso: e.target.value }); 
      showToast(`ISO set to ${e.target.value}`);
    } 
    catch (err) { 
      console.error("[IPC] ISO Error:", err);
      showToast(`Failed to set ISO: ${err}`, 'error'); 
    }
  });

  apertureSelect.addEventListener("change", async (e) => {
    console.log("[IPC] Aperture Change:", e.target.value);
    try { 
      await invoke("setaperture", { aperture: e.target.value }); 
      showToast(`Aperture set to f/${e.target.value}`);
    } 
    catch (err) { 
      console.error("[IPC] Aperture Error:", err);
      showToast(`Failed to set Aperture: ${err}`, 'error'); 
    }
  });

  const shutterSelect = document.querySelector("#shutter-select");
  shutterSelect.addEventListener("change", async (e) => {
    try {
      await invoke("setshutterspeed", { speed: e.target.value });
      showToast(`Shutter set to ${e.target.value}`);
    } catch (err) {
      showToast(`Failed to set Shutter: ${err}`, 'error');
    }
  });

  const focusStepSelect = document.querySelector("#focus-step-select");
  
  document.querySelector("#focus-near-btn").addEventListener("click", async () => {
    try {
      const step = parseInt(focusStepSelect.value);
      await invoke("manualfocus", { direction: -step });
      showToast(`Focused Near (Step ${step})`);
    } catch (err) {
      showToast(`Focus failed: ${err}`, 'error');
    }
  });

  document.querySelector("#focus-far-btn").addEventListener("click", async () => {
    try {
      const step = parseInt(focusStepSelect.value);
      await invoke("manualfocus", { direction: step });
      showToast(`Focused Far (Step ${step})`);
    } catch (err) {
      showToast(`Focus failed: ${err}`, 'error');
    }
  });

  const resolutionSelect = document.querySelector("#resolution-select");
  resolutionSelect.addEventListener("change", async (e) => {
    showToast(`Resolution set to ${e.target.value === 'large' ? '1056x704' : '528x352'}`);
  });

  const wbSelect = document.querySelector("#wb-select");
  wbSelect.addEventListener("change", async (e) => {
    try {
      await invoke("setwhitebalance", { wb: e.target.value });
      showToast(`White Balance: ${e.target.value}`);
    } catch (err) {
      showToast(`WB failed: ${err}`, 'error');
    }
  });

  const styleSelect = document.querySelector("#style-select");
  styleSelect.addEventListener("change", async (e) => {
    try {
      await invoke("setpicturestyle", { style: e.target.value });
      showToast(`Picture Style: ${e.target.value}`);
    } catch (err) {
      showToast(`Style failed: ${err}`, 'error');
    }
  });

  const exposureSelect = document.querySelector("#exposure-select");
  exposureSelect.addEventListener("change", async (e) => {
    try {
      await invoke("setexposurecompensation", { ev: e.target.value });
      showToast(`Exposure Comp: ${e.target.value} EV`);
    } catch (err) {
      showToast(`Exposure failed: ${err}`, 'error');
    }
  });

  const meteringSelect = document.querySelector("#metering-select");
  meteringSelect.addEventListener("change", async (e) => {
    try {
      await invoke("setmeteringmode", { mode: e.target.value });
      showToast(`Metering: ${e.target.value}`);
    } catch (err) {
      showToast(`Metering failed: ${err}`, 'error');
    }
  });

  const flashSelect = document.querySelector("#flash-select");
  flashSelect.addEventListener("change", async (e) => {
    try {
      await invoke("setflashmode", { mode: e.target.value });
      showToast(`Flash: ${e.target.value}`);
    } catch (err) {
      showToast(`Flash failed: ${err}`, 'error');
    }
  });

  // ═══════════════════════════════════════════════════════
  //  AI AUTOFOCUS SYSTEM — MediaPipe Face Detection → Camera AF
  // ═══════════════════════════════════════════════════════

  const focusModeSelect = document.querySelector("#focus-mode-select");
  const focusBox = document.querySelector("#focus-box");

  async function throttledAF(reason, xPct = 0.5, yPct = 0.5) {
    const now = Date.now();
    if (afInFlight || (now - lastAfTriggerTime) < AF_COOLDOWN_MS) return;
    afInFlight = true;
    lastAfTriggerTime = now;
    
    try {
      console.log(`[AI-AF] Targeted AF at ${xPct.toFixed(2)}, ${yPct.toFixed(2)}: ${reason}`);
      showToast("Smart Focus: Target locked...");
      
      // Calculate absolute sensor coordinates for Canon 600D (5184 x 3456)
      // The camera will move the live view zoom box (AF point) to this location!
      const cx = Math.floor(xPct * 5184);
      const cy = Math.floor(yPct * 3456);
      
      // Move AF point to face/object
      await invoke("setzoomposition", { x: cx, y: cy });
      
      // Wait a tiny bit for the camera to register the box move
      await new Promise(r => setTimeout(r, 150));
      
      // Hardware Contrast AF at the new location
      await invoke("triggerautofocus");
      
      console.log(`[AI-AF] Hardware AF Triggered at ${cx}, ${cy}`);
    } catch (e) {
      console.warn(`[AI-AF] Failed: ${e}`);
    } finally {
      afInFlight = false;
    }
  }

  // ── Face tracking state ──
  let lastFaceX = -1;
  let lastFaceY = -1;
  let faceVisible = false;
  let frameCount = 0;

  focusModeSelect.addEventListener("change", async (e) => {
    const mode = e.target.value;
    try {
      if (mode === "AI Tracking") {
          showToast("AI Face Tracking — AF triggers when face moves");
      } else if (mode === "Smart Focus") {
          showToast("Smart Eye-Priority — High-confidence AF");
      } else if (mode === "Manual Object") {
          showToast("Manual Object Tracking — Click to target");
      } else {
          // "Manual" or "One Shot" — real gphoto2 focus modes
          await invoke("setfocusmode", { mode });
          showToast(`Focus Mode: ${mode}`);
          // Hide focus box when leaving AI modes
          focusBox.style.display = 'none';
          focusBox.classList.remove("eye-priority");
      }
    } catch (err) {
      showToast(`Mode change failed: ${err}`, 'error');
    }
  });

  document.querySelector("#trigger-af-btn").addEventListener("click", async () => {
    try {
      await invoke("triggerautofocus");
      showToast(`Autofocus Triggered`);
    } catch (err) {
      showToast(`AF failed: ${err}`, 'error');
    }
  });

  document.querySelector("#debug-config-btn").addEventListener("click", async () => {
    try {
      await invoke("debugcamera");
      showToast(`Camera config dumped to terminal`);
    } catch (err) {
      showToast(`Debug failed: ${err}`, 'error');
    }
  });

  // ── MediaPipe Face Detection — Init ──
  let faceDetection = null;
  let aiReady = false;
  try {
    console.log("[AI] Testing asset fetch...");
    try {
        const testRes = await fetch('mediapipe/face_detection_short_range.tflite');
        const testText = await testRes.clone().text();
        console.log(`[AI] tflite fetch: ${testRes.status} ${testRes.headers.get('content-type')} (Length: ${testText.length})`);
        if (testText.startsWith('<!DOCTYPE html>') || testText.includes('<html>')) {
            throw new Error(`Tauri dev server returned HTML instead of the model file! Path issue.`);
        }
    } catch (err) {
        showToast(`Asset Fetch Error: ${err.message}`, 'error');
    }

    console.log("[AI] FaceDetection type: " + (typeof FaceDetection));
    if (typeof FaceDetection !== 'undefined') {
        faceDetection = new FaceDetection({
            locateFile: (file) => {
                console.log(`[AI] locateFile requested: ${file}`);
                return `mediapipe/${file}`;
            },
        });

        faceDetection.setOptions({
            model: 'short',
            minDetectionConfidence: 0.5,
        });

        // Register the results callback BEFORE initialize()
        faceDetection.onResults(handleFaceResults);

        // Initialize the WASM runtime
        console.log("[AI] Calling initialize()...");
        await faceDetection.initialize();
        aiReady = true;
        console.log("[AI] ✓ Face Detection ready!");
        showToast("AI Face Detection loaded");
    } else {
        console.warn("[AI] MediaPipe FaceDetection not found. AI features disabled.");
    }
  } catch (e) {
    console.error("[AI] Failed to init MediaPipe:", e);
    showToast(`AI Init Error: ${e.message || e}`, 'error');
  }

  let aiFrameCounter = 0;
  function handleFaceResults(results) {
    aiFrameCounter++;
    if (aiFrameCounter % 10 === 0) {
        console.warn(`[AI-STATS] Processed ${aiFrameCounter} frames. Faces found: ${results.detections?.length || 0}`);
    }

    const currentMode = focusModeSelect.value;
    const isAiMode = (currentMode === "AI Tracking" || currentMode === "Smart Focus");

    if (!results.detections || results.detections.length === 0) {
      // No face detected
      if (faceVisible && isAiMode) {
        focusBox.style.display = 'none';
        focusBox.classList.remove("locked", "focusing", "eye-priority");
        faceVisible = false;
      }
      return;
    }

    // Use the first (highest-confidence) detection
    const det = results.detections[0];
    const bbox = det.boundingBox;
    
    // DEBUG DUMP: let's see exactly what the bounding box object looks like
    if (frameCount % 16 === 0) {
        console.warn(`[AI-BBOX] ${JSON.stringify(bbox)}`);
    }

    // MediaPipe bounding box format fallback to prevent NaN CSS values
    const w = bbox.width || 0.2;
    const h = bbox.height || 0.2;
    const x = bbox.xCenter ?? (bbox.originX ? bbox.originX + w / 2 : (bbox.xMin ? bbox.xMin + w / 2 : 0.5));
    const y = bbox.yCenter ?? (bbox.originY ? bbox.originY + h / 2 : (bbox.yMin ? bbox.yMin + h / 2 : 0.5));

    // ── Update portrait blur mask position ──
    previewWrapperEl.style.setProperty("--face-x", `${x * 100}%`);
    previewWrapperEl.style.setProperty("--face-y", `${y * 100}%`);

    if (!isAiMode) return; // Only act when in an AI focus mode

    // ── Show focus box on detected face ──
    const wrapRect = previewWrapperEl.getBoundingClientRect();
    const boxSize = Math.max(w, h) * Math.max(wrapRect.width, wrapRect.height);
    const finalBoxSize = Math.max(boxSize || 100, 50); // Fallback to 100px if NaN
    
    focusBox.style.width = `${finalBoxSize}px`;
    focusBox.style.height = `${finalBoxSize}px`;
    focusBox.style.left = `calc(${x * 100}% - ${finalBoxSize / 2}px)`;
    focusBox.style.top = `calc(${y * 100}% - ${finalBoxSize / 2}px)`;
    focusBox.style.display = 'block';

    // ── Determine if AF should fire ──
    const dx = Math.abs(x - lastFaceX);
    const dy = Math.abs(y - lastFaceY);
    const faceMoved = (dx > 0.05 || dy > 0.05); // >5% of frame
    const isNewFace = !faceVisible;

    if (currentMode === "Smart Focus") {
      focusBox.classList.add("eye-priority");
      // Only trigger on high-confidence detections + movement
      const score = det.categories?.[0]?.score ?? (det.score?.[0] ?? 0.8);
      if (score > 0.75 && (faceMoved || isNewFace)) {
        focusBox.classList.remove("locked");
        focusBox.classList.add("focusing");
        throttledAF(`face confidence=${score.toFixed(2)}, moved=${faceMoved}`, x, y);
        lastFaceX = x;
        lastFaceY = y;
        setTimeout(() => {
          focusBox.classList.remove("focusing");
          focusBox.classList.add("locked");
        }, 800);
      }
    } else if (currentMode === "AI Tracking") {
      focusBox.classList.remove("eye-priority");
      if (faceMoved || isNewFace) {
        focusBox.classList.remove("locked");
        focusBox.classList.add("focusing");
        throttledAF(`face moved dx=${dx.toFixed(3)} dy=${dy.toFixed(3)}`, x, y);
        lastFaceX = x;
        lastFaceY = y;
        setTimeout(() => {
          focusBox.classList.remove("focusing");
          focusBox.classList.add("locked");
        }, 800);
      }
    }

    faceVisible = true;
  }

  // ── Offscreen canvas for feeding frames to MediaPipe ──
  const offscreenCanvas = document.createElement('canvas');
  const ctx = offscreenCanvas.getContext('2d');

  let prevCenterPixels = null;

  let aiProcessing = false;
  async function processAiFrame(imgElement) {
    if (!aiReady || aiProcessing) return;

    aiProcessing = true;
    try {
      // Ensure the image is fully decoded before passing to canvas
      await imgElement.decode();
      
      if (imgElement.naturalWidth === 0) {
          aiProcessing = false;
          return;
      }

      offscreenCanvas.width = imgElement.naturalWidth;
      offscreenCanvas.height = imgElement.naturalHeight;
      ctx.drawImage(imgElement, 0, 0);

      const currentMode = focusModeSelect.value;
      if (currentMode === "Manual Object") {
          // ── Fast Manual Object Motion AF ──
          const cx = Math.floor(offscreenCanvas.width * manualObjPctX);
          const cy = Math.floor(offscreenCanvas.height * manualObjPctY);
          const cw = Math.floor(offscreenCanvas.width * 0.15);
          const ch = Math.floor(offscreenCanvas.height * 0.15);
          
          const sx = Math.max(0, Math.min(offscreenCanvas.width - cw, cx - Math.floor(cw/2)));
          const sy = Math.max(0, Math.min(offscreenCanvas.height - ch, cy - Math.floor(ch/2)));
          
          const imgData = ctx.getImageData(sx, sy, cw, ch).data;
          
          // Show a fixed box at clicked position
          focusBox.style.width = '15%';
          focusBox.style.height = '15%';
          focusBox.style.left = `${(manualObjPctX * 100) - 7.5}%`;
          focusBox.style.top = `${(manualObjPctY * 100) - 7.5}%`;
          focusBox.style.display = 'block';
          focusBox.classList.remove("eye-priority", "locked");

          if (prevCenterPixels) {
              let diff = 0;
              // Subsample pixels for speed (check every 16th pixel)
              for (let i = 0; i < imgData.length; i += 64) {
                  diff += Math.abs(imgData[i] - prevCenterPixels[i]);
                  diff += Math.abs(imgData[i+1] - prevCenterPixels[i+1]);
                  diff += Math.abs(imgData[i+2] - prevCenterPixels[i+2]);
              }
              const avgDiff = diff / (imgData.length / 64) / 3;
              
              if (avgDiff > 15) { // Threshold for object motion
                  focusBox.classList.add("focusing");
                  throttledAF(`Manual Object Motion (${avgDiff.toFixed(1)})`, manualObjPctX, manualObjPctY);
                  setTimeout(() => {
                      focusBox.classList.remove("focusing");
                      focusBox.classList.add("locked");
                  }, 800);
              }
          }
          prevCenterPixels = new Uint8Array(imgData);

      } else if (faceDetection && (currentMode === "AI Tracking" || currentMode === "Smart Focus")) {
          // ── MediaPipe Face Detection ──
          await faceDetection.send({ image: offscreenCanvas });
      }

    } catch (e) {
      // e.g., decode errors or MediaPipe exceptions
      // console.warn("[AI] Frame processing skipped/error:", e);
    } finally {
      aiProcessing = false;
    }
  }

  // ── UI Toggles ──
  const peakingToggle = document.querySelector("#peaking-toggle");
  peakingToggle.addEventListener("change", (e) => {
    if (e.target.checked) {
        previewImgEl.classList.add("peaking-active");
        showToast("Focus Peaking Enabled");
    } else {
        previewImgEl.classList.remove("peaking-active");
    }
  });

  previewWrapperEl = document.querySelector("#preview-wrapper");
  const previewBgEl = document.querySelector("#preview-background");
  const portraitToggle = document.querySelector("#portrait-toggle");
  const portraitSlider = document.querySelector("#portrait-blur-slider");
  const portraitControls = document.querySelector("#portrait-controls");

  portraitToggle.addEventListener("change", (e) => {
    if (e.target.checked) {
        previewImgEl.classList.add("portrait-active");
        previewBgEl.classList.add("active");
        portraitControls.style.display = "block";
        showToast("Portrait Mode Active");
    } else {
        previewImgEl.classList.remove("portrait-active");
        previewBgEl.classList.remove("active");
        portraitControls.style.display = "none";
    }
  });

  portraitSlider.addEventListener("input", (e) => {
    previewWrapperEl.style.setProperty("--blur-amount", `${e.target.value}px`);
  });

  // Listen for Live View frames emitted from Rust
  await listen('preview-frame', async (event) => {
    const b64Data = event.payload;
    const dataUrl = `data:image/jpeg;base64,${b64Data}`;
    previewImgEl.src = dataUrl;
    if (previewBgEl) previewBgEl.src = dataUrl;

    // Run AI tracking every 8th frame (~4 FPS at 30 FPS stream)
    // Only when in an AI/Tracking focus mode, to save CPU
    const currentMode = focusModeSelect.value;
    const isAiMode = (currentMode === "AI Tracking" || currentMode === "Smart Focus" || currentMode === "Manual Object");
    if (isAiMode && frameCount % 8 === 0) {
        processAiFrame(previewImgEl);
    }
    frameCount++;

    if (previewImgEl.style.opacity == 0 || previewImgEl.style.opacity === "") {
        previewImgEl.style.opacity = 1;
        document.querySelector("#preview-placeholder").style.display = "none";
    }
  });

  console.log("[APP] UI Initialized successfully.");
  } catch (err) {
    console.error("[APP] Initialization CRASH:", err);
    const body = document.body;
    const errorDiv = document.createElement("div");
    errorDiv.style = "position:fixed;top:0;left:0;width:100%;background:red;color:white;padding:10px;z-index:9999;text-align:center;";
    errorDiv.innerText = "STARTUP ERROR: " + err.message;
    body.appendChild(errorDiv);
  }

  let lastStatus = "";
  listen("camera-status", (event) => {
    const status = event.payload;
    // Only show meaningful status changes, skip per-command "set OK" noise
    const isSettingAck = status.endsWith("set OK");
    if (status !== lastStatus && !isSettingAck) {
      showToast(`Camera: ${status}`);
      lastStatus = status;
    }
    const streamStatus = document.querySelector(".status-indicator span");
    if (streamStatus) {
        streamStatus.textContent = status === "Connected" ? "Live" : status;
        streamStatus.parentElement.className = `status-indicator ${status === "Connected" ? "status-live" : "status-idle"}`;
    }
  });

  listen("camera-error", (event) => {
    showToast(event.payload, 'error');
  });
  
  // Add CSS for spinner if not present
  if (!document.querySelector('#spinner-style')) {
    const style = document.createElement('style');
    style.id = 'spinner-style';
    style.innerHTML = `
      @keyframes spin { 100% { transform: rotate(360deg); } }
      .spinner { animation: spin 1s linear infinite; }
      .text-error { color: var(--error); }
      .text-success { color: var(--success); }
    `;
    document.head.appendChild(style);
  }
});

console.log("[APP] main.js fully loaded.");
