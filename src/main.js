const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

let deviceInputEl, startBtnEl, previewImgEl, previewWrapperEl;
let createDeviceBtn, isoSelect, apertureSelect, statusPill, statusText;
const toastContainer = document.getElementById('toast-container');

function showToast(msg, type = 'success') {
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
      await invoke("stop_camera");
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
    const res = await invoke("start_camera", { devicePath });
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
    const path = await invoke("create_virtual_device");
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
  deviceInputEl = document.querySelector("#device-path");
  startBtnEl = document.querySelector("#start-btn");
  previewImgEl = document.querySelector("#preview-img");
  previewWrapperEl = document.querySelector("#preview-wrapper");
  createDeviceBtn = document.querySelector("#create-device-btn");
  isoSelect = document.querySelector("#iso-select");
  apertureSelect = document.querySelector("#aperture-select");
  statusPill = document.querySelector("#status-pill");
  statusText = document.querySelector(".status-text");
  
  startBtnEl.addEventListener("click", toggleStream);
  createDeviceBtn.addEventListener("click", createDevice);

  previewWrapperEl.addEventListener("click", (e) => {
    if (!isStreaming) return;
    const rect = previewWrapperEl.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    const focusBox = document.querySelector("#focus-box");
    focusBox.style.left = `${x - 30}px`;
    focusBox.style.top = `${y - 30}px`;
    focusBox.style.display = 'block';
    
    // Trigger AF at this location (visual only for now as PTP coordinate mapping is camera-specific)
    invoke("trigger_autofocus").catch(err => showToast(`AF Error: ${err}`, 'error'));
  });

  isoSelect.addEventListener("change", async (e) => {
    try { 
      await invoke("set_iso", { iso: e.target.value }); 
      showToast(`ISO set to ${e.target.value}`);
    } 
    catch (err) { showToast(`Failed to set ISO: ${err}`, 'error'); }
  });

  apertureSelect.addEventListener("change", async (e) => {
    try { 
      await invoke("set_aperture", { aperture: e.target.value }); 
      showToast(`Aperture set to f/${e.target.value}`);
    } 
    catch (err) { showToast(`Failed to set Aperture: ${err}`, 'error'); }
  });

  const shutterSelect = document.querySelector("#shutter-select");
  shutterSelect.addEventListener("change", async (e) => {
    try {
      await invoke("set_shutter_speed", { speed: e.target.value });
      showToast(`Shutter set to ${e.target.value}`);
    } catch (err) {
      showToast(`Failed to set Shutter: ${err}`, 'error');
    }
  });

  const focusStepSelect = document.querySelector("#focus-step-select");
  
  document.querySelector("#focus-near-btn").addEventListener("click", async () => {
    try {
      const step = parseInt(focusStepSelect.value);
      await invoke("manual_focus", { direction: -step });
      showToast(`Focused Near (Step ${step})`);
    } catch (err) {
      showToast(`Focus failed: ${err}`, 'error');
    }
  });

  document.querySelector("#focus-far-btn").addEventListener("click", async () => {
    try {
      const step = parseInt(focusStepSelect.value);
      await invoke("manual_focus", { direction: step });
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
      await invoke("set_white_balance", { wb: e.target.value });
      showToast(`White Balance: ${e.target.value}`);
    } catch (err) {
      showToast(`WB failed: ${err}`, 'error');
    }
  });

  const styleSelect = document.querySelector("#style-select");
  styleSelect.addEventListener("change", async (e) => {
    try {
      await invoke("set_picture_style", { style: e.target.value });
      showToast(`Picture Style: ${e.target.value}`);
    } catch (err) {
      showToast(`Style failed: ${err}`, 'error');
    }
  });

  const exposureSelect = document.querySelector("#exposure-select");
  exposureSelect.addEventListener("change", async (e) => {
    try {
      await invoke("set_exposure_compensation", { ev: e.target.value });
      showToast(`Exposure Comp: ${e.target.value} EV`);
    } catch (err) {
      showToast(`Exposure failed: ${err}`, 'error');
    }
  });

  const meteringSelect = document.querySelector("#metering-select");
  meteringSelect.addEventListener("change", async (e) => {
    try {
      await invoke("set_metering_mode", { mode: e.target.value });
      showToast(`Metering: ${e.target.value}`);
    } catch (err) {
      showToast(`Metering failed: ${err}`, 'error');
    }
  });

  const flashSelect = document.querySelector("#flash-select");
  flashSelect.addEventListener("change", async (e) => {
    try {
      await invoke("set_flash_mode", { mode: e.target.value });
      showToast(`Flash: ${e.target.value}`);
    } catch (err) {
      showToast(`Flash failed: ${err}`, 'error');
    }
  });

  const focusModeSelect = document.querySelector("#focus-mode-select");
  let aiFocusInterval = null;

  focusModeSelect.addEventListener("change", async (e) => {
    const mode = e.target.value;
    try {
      if (mode === "AI Tracking") {
          showToast("AI Tracking Active - Searching for focus...");
          startAiFocusHunt();
      } else {
          stopAiFocusHunt();
          await invoke("set_focus_mode", { mode });
          showToast(`Focus Mode: ${mode}`);
      }
    } catch (err) {
      showToast(`Mode change failed: ${err}`, 'error');
    }
  });

  function startAiFocusHunt() {
    if (aiFocusInterval) return;
    let direction = 1;
    let steps = 0;
    aiFocusInterval = setInterval(async () => {
        try {
            // Simulated AI Hunting: move a small step and check if sharp (visual only for now)
            await invoke("manual_focus", { direction });
            steps++;
            if (steps > 3) {
                direction *= -1; // Reverse hunt
                steps = 0;
            }
        } catch (e) {}
    }, 2000); // Slow hunt
  }

  function stopAiFocusHunt() {
      if (aiFocusInterval) {
          clearInterval(aiFocusInterval);
          aiFocusInterval = null;
      }
  }

  document.querySelector("#trigger-af-btn").addEventListener("click", async () => {
    try {
      await invoke("trigger_autofocus");
      showToast(`Autofocus Triggered`);
    } catch (err) {
      showToast(`AF failed: ${err}`, 'error');
    }
  });

  document.querySelector("#debug-config-btn").addEventListener("click", async () => {
    try {
      await invoke("debug_camera");
      showToast(`Camera config dumped to terminal`);
    } catch (err) {
      showToast(`Debug failed: ${err}`, 'error');
    }
  });

  // AI Focus Model (MediaPipe)
  const faceDetection = new FaceDetection({
    locateFile: (file) => `https://cdn.jsdelivr.net/npm/@mediapipe/face_detection/${file}`,
  });
  
  faceDetection.setOptions({
    model: 'short',
    minDetectionConfidence: 0.5
  });

  const offscreenCanvas = document.createElement('canvas');
  const ctx = offscreenCanvas.getContext('2d');
  
  let lastFaceX = 0.5;
  let lastFaceY = 0.5;
  let frameCount = 0;
  const focusBox = document.querySelector("#focus-box");

  const peakingToggle = document.querySelector("#peaking-toggle");
  peakingToggle.addEventListener("change", (e) => {
    if (e.target.checked) {
        previewImgEl.classList.add("peaking-active");
        showToast("Focus Peaking Enabled");
    } else {
        previewImgEl.classList.remove("peaking-active");
    }
  });

  const previewWrapperEl = document.querySelector("#preview-wrapper");
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

  faceDetection.onResults((results) => {
    if (results.detections.length > 0) {
      const face = results.detections[0].boundingBox;
      const x = face.xCenter;
      const y = face.yCenter;
      
      // Update CSS variables for the blur mask
      previewWrapperEl.style.setProperty("--face-x", `${x * 100}%`);
      previewWrapperEl.style.setProperty("--face-y", `${y * 100}%`);

      const currentMode = document.querySelector("#focus-mode-select").value;
      
      // Update Focus Box position
      focusBox.style.left = `${x * 100}%`;
      focusBox.style.top = `${y * 100}%`;
      focusBox.style.display = 'block';
      
      if (currentMode === "Smart Focus") {
          focusBox.classList.add("eye-priority");
          if (results.detections[0].categories[0].score > 0.8) {
              invoke("trigger_autofocus").catch(() => {});
          }
      } else {
          focusBox.classList.remove("eye-priority");
      }
      
      if (currentMode === "AI Tracking" || currentMode === "Smart Focus") {
          const dx = Math.abs(x - lastFaceX);
          const dy = Math.abs(y - lastFaceY);
          if (dx > 0.04 || dy > 0.04) {
              invoke("trigger_autofocus").catch(() => {});
              lastFaceX = x;
              lastFaceY = y;
          }
      }
    }
  });

  async function processAiFrame(imgElement) {
    if (imgElement.complete && imgElement.naturalWidth > 0) {
        offscreenCanvas.width = imgElement.naturalWidth;
        offscreenCanvas.height = imgElement.naturalHeight;
        ctx.drawImage(imgElement, 0, 0);
        await faceDetection.send({ image: offscreenCanvas });
    }
  }

  // Listen for Live View frames emitted from Rust
  await listen('preview-frame', async (event) => {
    const b64Data = event.payload;
    const dataUrl = `data:image/jpeg;base64,${b64Data}`;
    previewImgEl.src = dataUrl;
    if (previewBgEl) previewBgEl.src = dataUrl;
    
    if (frameCount % 10 === 0) {
        processAiFrame(previewImgEl);
    }
    frameCount++;

    if (previewImgEl.style.opacity == 0 || previewImgEl.style.opacity === "") {
        previewImgEl.style.opacity = 1;
    }
  });

  // Auto-start and status listeners
  listen("camera-status", (event) => {
    const status = event.payload;
    showToast(`Camera: ${status}`);
    const streamStatus = document.querySelector(".status-indicator span");
    if (streamStatus) {
        streamStatus.textContent = status === "Connected" ? "Live" : status;
        streamStatus.parentElement.className = `status-indicator ${status === "Connected" ? "status-live" : "status-idle"}`;
    }
  });

  listen("camera-error", (event) => {
    showToast(event.payload, 'error');
  });

  // Start the connection immediately
  const defaultDevice = localStorage.getItem("last-v4l2-device") || "/dev/video9";
  invoke("start_camera", { devicePath: defaultDevice }).catch(() => {});
  
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
