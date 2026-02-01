/**
 * Lince Spirit - An interactive lynx that lives in your website
 * 
 * The lynx has moods and behaviors:
 * - IDLE: Peacefully observing, occasional ear twitches
 * - PLAYFUL: Nudging buttons, pawing at elements
 * - CURIOUS: Following the cursor, investigating clicks
 * - HUNGRY: Trying to "eat" parts of the navbar
 * - MISCHIEVOUS: Blocking content, demanding attention
 */

(function() {
    'use strict';

    // ========== Configuration ==========
    const CONFIG = {
        // Timing (in ms)
        idleMinDuration: 8000,
        idleMaxDuration: 15000,
        actionDuration: 3000,
        mischievousChance: 0.08,    // 8% chance to block content
        nudgeChance: 0.15,          // 15% chance to nudge a button
        eatChance: 0.10,            // 10% chance to try eating navbar
        
        // Movement
        moveSpeed: 0.08,
        bounceAmplitude: 3,
        
        // Spawn position
        startX: 60,
        startY: 100,
    };

    // ========== Lynx State ==========
    const state = {
        mood: 'idle',
        x: CONFIG.startX,
        y: CONFIG.startY,
        targetX: CONFIG.startX,
        targetY: CONFIG.startY,
        isMoving: false,
        lastAction: Date.now(),
        actionTimeout: null,
        isBlocking: false,
        currentTarget: null,
        earTwitchInterval: null,
        tailSwayInterval: null,
        blinkInterval: null,
    };

    // ========== SVG Lynx Character ==========
    const createLynxSVG = () => {
        const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
        svg.setAttribute('viewBox', '0 0 120 100');
        svg.setAttribute('width', '120');
        svg.setAttribute('height', '100');
        svg.id = 'lince-spirit-svg';
        
        // Tribal-style lynx inspired by the logo
        svg.innerHTML = `
            <defs>
                <style>
                    .lince-body { fill: currentColor; }
                    .lince-accent { fill: none; stroke: currentColor; stroke-width: 2; }
                    .lince-eye { fill: currentColor; }
                    .lince-eye-pupil { fill: var(--color-bg, #000); }
                    .lince-whisker { stroke: currentColor; stroke-width: 1.5; fill: none; }
                    .lince-ear-tuft { fill: currentColor; }
                    #lince-left-ear, #lince-right-ear { 
                        transform-origin: center bottom;
                        transition: transform 0.15s ease;
                    }
                    #lince-tail {
                        transform-origin: 15px 75px;
                        transition: transform 0.3s ease;
                    }
                    #lince-left-eyelid, #lince-right-eyelid {
                        transform-origin: center;
                        transition: transform 0.1s ease;
                    }
                    .lince-paw {
                        transform-origin: center top;
                    }
                    #lince-left-paw {
                        animation: none;
                    }
                </style>
            </defs>
            
            <!-- Tail -->
            <g id="lince-tail">
                <path class="lince-body" d="M15 75 Q5 65 10 50 Q15 40 20 45 Q25 55 20 70 Q18 75 15 75Z"/>
                <path class="lince-accent" d="M12 60 L15 55 M17 52 L14 48"/>
            </g>
            
            <!-- Body -->
            <ellipse class="lince-body" cx="60" cy="70" rx="35" ry="22"/>
            
            <!-- Tribal pattern on body -->
            <path class="lince-accent" d="M35 65 L45 60 L50 65 M55 58 L60 55 L65 58 M70 60 L80 65"/>
            <path class="lince-accent" d="M40 75 Q50 72 60 75 Q70 72 80 75"/>
            
            <!-- Back legs (visible) -->
            <ellipse class="lince-body" cx="80" cy="82" rx="8" ry="12"/>
            <ellipse class="lince-body" cx="40" cy="82" rx="8" ry="12"/>
            
            <!-- Front paws -->
            <g id="lince-left-paw" class="lince-paw">
                <ellipse class="lince-body" cx="45" cy="88" rx="6" ry="4"/>
                <path class="lince-accent" d="M41 90 L43 88 M45 90 L45 87 M49 90 L47 88"/>
            </g>
            <g id="lince-right-paw" class="lince-paw">
                <ellipse class="lince-body" cx="75" cy="88" rx="6" ry="4"/>
                <path class="lince-accent" d="M71 90 L73 88 M75 90 L75 87 M79 90 L77 88"/>
            </g>
            
            <!-- Head -->
            <ellipse class="lince-body" cx="60" cy="35" rx="28" ry="24"/>
            
            <!-- Tribal face patterns -->
            <path class="lince-accent" d="M45 25 L50 30 L45 35"/>
            <path class="lince-accent" d="M75 25 L70 30 L75 35"/>
            <path class="lince-accent" d="M55 45 L60 50 L65 45"/>
            
            <!-- Ears -->
            <g id="lince-left-ear">
                <polygon class="lince-body" points="35,20 42,5 50,22"/>
                <polygon class="lince-ear-tuft" points="42,5 44,0 46,5 44,8"/>
                <path class="lince-accent" d="M40 18 L43 10 L46 18"/>
            </g>
            <g id="lince-right-ear">
                <polygon class="lince-body" points="85,20 78,5 70,22"/>
                <polygon class="lince-ear-tuft" points="78,5 76,0 74,5 76,8"/>
                <path class="lince-accent" d="M80 18 L77 10 L74 18"/>
            </g>
            
            <!-- Eyes -->
            <g id="lince-left-eye">
                <ellipse class="lince-eye" cx="48" cy="32" rx="6" ry="5"/>
                <ellipse class="lince-eye-pupil" id="lince-left-pupil" cx="48" cy="32" rx="3" ry="4"/>
                <ellipse id="lince-left-eyelid" class="lince-body" cx="48" cy="28" rx="7" ry="3" style="transform: scaleY(0);"/>
            </g>
            <g id="lince-right-eye">
                <ellipse class="lince-eye" cx="72" cy="32" rx="6" ry="5"/>
                <ellipse class="lince-eye-pupil" id="lince-right-pupil" cx="72" cy="32" rx="3" ry="4"/>
                <ellipse id="lince-right-eyelid" class="lince-body" cx="72" cy="28" rx="7" ry="3" style="transform: scaleY(0);"/>
            </g>
            
            <!-- Nose -->
            <polygon class="lince-body" points="60,38 56,42 64,42"/>
            <circle cx="60" cy="40" r="2" style="fill: var(--color-bg, #000);"/>
            
            <!-- Whiskers -->
            <g id="lince-whiskers">
                <path class="lince-whisker" d="M50 42 Q35 38 25 35"/>
                <path class="lince-whisker" d="M50 44 Q35 44 22 44"/>
                <path class="lince-whisker" d="M50 46 Q35 50 25 55"/>
                <path class="lince-whisker" d="M70 42 Q85 38 95 35"/>
                <path class="lince-whisker" d="M70 44 Q85 44 98 44"/>
                <path class="lince-whisker" d="M70 46 Q85 50 95 55"/>
            </g>
            
            <!-- Mouth (changes with mood) -->
            <path id="lince-mouth" class="lince-accent" d="M55 48 Q60 52 65 48" style="stroke-width: 1.5;"/>
            
            <!-- Cheek tufts (tribal style) -->
            <polygon class="lince-body" points="32,40 28,50 35,48 32,55 38,50"/>
            <polygon class="lince-body" points="88,40 92,50 85,48 88,55 82,50"/>
        `;
        
        return svg;
    };

    // ========== Create Container ==========
    const createLynxContainer = () => {
        const container = document.createElement('div');
        container.id = 'lince-spirit';
        container.innerHTML = `
            <div id="lince-spirit-body">
                ${createLynxSVG().outerHTML}
            </div>
            <div id="lince-spirit-speech" class="hidden"></div>
        `;
        document.body.appendChild(container);
        
        // Add blocking overlay (hidden by default)
        const overlay = document.createElement('div');
        overlay.id = 'lince-block-overlay';
        overlay.className = 'hidden';
        overlay.innerHTML = `
            <div id="lince-block-message">
                <span id="lince-block-text">*yawns and stretches*</span>
                <button id="lince-block-dismiss">Pet the lynx</button>
            </div>
        `;
        document.body.appendChild(overlay);
        
        return container;
    };

    // ========== Animation Functions ==========
    
    const twitchEar = (ear) => {
        const earEl = document.getElementById(`lince-${ear}-ear`);
        if (!earEl) return;
        
        const angle = ear === 'left' ? -15 : 15;
        earEl.style.transform = `rotate(${angle}deg)`;
        setTimeout(() => {
            earEl.style.transform = 'rotate(0deg)';
        }, 150);
    };

    const swayTail = () => {
        const tail = document.getElementById('lince-tail');
        if (!tail) return;
        
        const angle = Math.sin(Date.now() / 300) * 20;
        tail.style.transform = `rotate(${angle}deg)`;
    };

    const blink = () => {
        const leftLid = document.getElementById('lince-left-eyelid');
        const rightLid = document.getElementById('lince-right-eyelid');
        if (!leftLid || !rightLid) return;
        
        leftLid.style.transform = 'scaleY(1) translateY(4px)';
        rightLid.style.transform = 'scaleY(1) translateY(4px)';
        
        setTimeout(() => {
            leftLid.style.transform = 'scaleY(0)';
            rightLid.style.transform = 'scaleY(0)';
        }, 120);
    };

    const lookAt = (x, y) => {
        const container = document.getElementById('lince-spirit');
        if (!container) return;
        
        const rect = container.getBoundingClientRect();
        const centerX = rect.left + rect.width / 2;
        const centerY = rect.top + rect.height / 3;
        
        const dx = x - centerX;
        const dy = y - centerY;
        const distance = Math.sqrt(dx * dx + dy * dy);
        
        const maxOffset = 2;
        const offsetX = (dx / Math.max(distance, 1)) * Math.min(distance / 50, maxOffset);
        const offsetY = (dy / Math.max(distance, 1)) * Math.min(distance / 50, maxOffset);
        
        const leftPupil = document.getElementById('lince-left-pupil');
        const rightPupil = document.getElementById('lince-right-pupil');
        
        if (leftPupil) leftPupil.setAttribute('cx', 48 + offsetX);
        if (leftPupil) leftPupil.setAttribute('cy', 32 + offsetY);
        if (rightPupil) rightPupil.setAttribute('cx', 72 + offsetX);
        if (rightPupil) rightPupil.setAttribute('cy', 32 + offsetY);
    };

    const setMouth = (mood) => {
        const mouth = document.getElementById('lince-mouth');
        if (!mouth) return;
        
        switch (mood) {
            case 'happy':
                mouth.setAttribute('d', 'M55 46 Q60 54 65 46');
                break;
            case 'mischievous':
                mouth.setAttribute('d', 'M54 48 Q60 52 66 48 L68 46');
                break;
            case 'hungry':
                mouth.setAttribute('d', 'M54 46 Q60 50 66 46 M57 50 L63 50');
                break;
            default:
                mouth.setAttribute('d', 'M55 48 Q60 52 65 48');
        }
    };

    const pawAnimation = (raised = true) => {
        const paw = document.getElementById('lince-left-paw');
        if (!paw) return;
        
        if (raised) {
            paw.style.transform = 'translateY(-10px) rotate(-20deg)';
        } else {
            paw.style.transform = 'translateY(0) rotate(0deg)';
        }
    };

    // ========== Movement ==========
    
    const moveTo = (x, y) => {
        state.targetX = x;
        state.targetY = y;
        state.isMoving = true;
    };

    const updatePosition = () => {
        if (!state.isMoving) return;
        
        const dx = state.targetX - state.x;
        const dy = state.targetY - state.y;
        const distance = Math.sqrt(dx * dx + dy * dy);
        
        if (distance < 2) {
            state.isMoving = false;
            return;
        }
        
        state.x += dx * CONFIG.moveSpeed;
        state.y += dy * CONFIG.moveSpeed;
        
        const container = document.getElementById('lince-spirit');
        if (container) {
            // Add slight bounce while moving
            const bounce = Math.sin(Date.now() / 100) * CONFIG.bounceAmplitude;
            container.style.left = `${state.x}px`;
            container.style.top = `${state.y + bounce}px`;
            
            // Face direction of movement
            if (dx < -5) {
                container.style.transform = 'scaleX(-1)';
            } else if (dx > 5) {
                container.style.transform = 'scaleX(1)';
            }
        }
    };

    // ========== Behaviors ==========
    
    const showSpeech = (text, duration = 2000) => {
        const speech = document.getElementById('lince-spirit-speech');
        if (!speech) return;
        
        speech.textContent = text;
        speech.classList.remove('hidden');
        
        setTimeout(() => {
            speech.classList.add('hidden');
        }, duration);
    };

    const nudgeButton = () => {
        const buttons = document.querySelectorAll('.btn, .navbar-item, .theme-toggle, .lang-btn');
        if (buttons.length === 0) return;
        
        const btn = buttons[Math.floor(Math.random() * buttons.length)];
        const rect = btn.getBoundingClientRect();
        
        // Move near the button
        moveTo(rect.left - 80, rect.top - 40 + window.scrollY);
        
        setTimeout(() => {
            pawAnimation(true);
            
            // Nudge the button
            const nudgeX = (Math.random() - 0.5) * 20;
            const nudgeY = (Math.random() - 0.5) * 10;
            
            btn.style.transition = 'transform 0.3s ease';
            btn.style.transform = `translate(${nudgeX}px, ${nudgeY}px)`;
            
            showSpeech(['*poke*', '*tap tap*', 'hehe~', '*boop*'][Math.floor(Math.random() * 4)]);
            
            setTimeout(() => {
                btn.style.transform = '';
                pawAnimation(false);
            }, 800);
        }, 1000);
    };

    const tryEatNavbar = () => {
        const navbar = document.querySelector('.navbar');
        if (!navbar) return;
        
        const rect = navbar.getBoundingClientRect();
        moveTo(rect.right - 150, rect.bottom + window.scrollY);
        
        setMouth('hungry');
        
        setTimeout(() => {
            const navItems = document.querySelectorAll('.navbar-item');
            if (navItems.length > 0) {
                const target = navItems[Math.floor(Math.random() * navItems.length)];
                
                // Chomping animation
                pawAnimation(true);
                showSpeech(['*nom nom*', '*chomp*', 'tasty...', '*nibble*'][Math.floor(Math.random() * 4)]);
                
                target.style.transition = 'opacity 0.3s ease, transform 0.3s ease';
                target.style.opacity = '0.5';
                target.style.transform = 'scale(0.95)';
                
                setTimeout(() => {
                    target.style.opacity = '';
                    target.style.transform = '';
                    pawAnimation(false);
                    setMouth('happy');
                }, 1500);
            }
        }, 1200);
    };

    const blockContent = () => {
        state.isBlocking = true;
        
        const overlay = document.getElementById('lince-block-overlay');
        const container = document.getElementById('lince-spirit');
        
        if (!overlay || !container) return;
        
        // Move to center
        const centerX = window.innerWidth / 2 - 60;
        const centerY = window.innerHeight / 2 - 50 + window.scrollY;
        moveTo(centerX, centerY);
        
        const messages = [
            { text: "*stretches across your screen*", action: "Give belly rubs" },
            { text: "*yawns dramatically*", action: "Pet the lynx" },
            { text: "Pay attention to me!", action: "Okay, okay!" },
            { text: "*flops down lazily*", action: "Scritch scritch" },
            { text: "I am the content now.", action: "Yes, you are" },
        ];
        
        const msg = messages[Math.floor(Math.random() * messages.length)];
        
        setTimeout(() => {
            setMouth('mischievous');
            document.getElementById('lince-block-text').textContent = msg.text;
            document.getElementById('lince-block-dismiss').textContent = msg.action;
            overlay.classList.remove('hidden');
            container.classList.add('blocking');
        }, 1000);
    };

    const dismissBlock = () => {
        const overlay = document.getElementById('lince-block-overlay');
        const container = document.getElementById('lince-spirit');
        
        if (overlay) overlay.classList.add('hidden');
        if (container) container.classList.remove('blocking');
        
        state.isBlocking = false;
        setMouth('happy');
        showSpeech(['*purrs*', '♡', 'mrrow~', '*happy noises*'][Math.floor(Math.random() * 4)], 1500);
        
        // Move to corner
        moveTo(60, 100 + window.scrollY);
    };

    const wanderAround = () => {
        const maxX = window.innerWidth - 150;
        const maxY = Math.min(window.innerHeight, document.body.scrollHeight) - 120;
        
        const newX = Math.random() * maxX;
        const newY = Math.random() * maxY + window.scrollY;
        
        moveTo(newX, newY);
    };

    // ========== Main Behavior Loop ==========
    
    const pickAction = () => {
        if (state.isBlocking) return;
        
        const roll = Math.random();
        
        if (roll < CONFIG.mischievousChance) {
            state.mood = 'mischievous';
            blockContent();
        } else if (roll < CONFIG.mischievousChance + CONFIG.eatChance) {
            state.mood = 'hungry';
            tryEatNavbar();
        } else if (roll < CONFIG.mischievousChance + CONFIG.eatChance + CONFIG.nudgeChance) {
            state.mood = 'playful';
            nudgeButton();
        } else {
            state.mood = 'idle';
            if (Math.random() > 0.5) {
                wanderAround();
            }
        }
        
        // Schedule next action
        const nextAction = CONFIG.idleMinDuration + 
            Math.random() * (CONFIG.idleMaxDuration - CONFIG.idleMinDuration);
        
        state.actionTimeout = setTimeout(pickAction, nextAction);
    };

    // ========== Event Handlers ==========
    
    const handleMouseMove = (e) => {
        lookAt(e.clientX, e.clientY);
    };

    const handleClick = (e) => {
        // Curious about clicks
        if (Math.random() > 0.7 && !state.isBlocking) {
            twitchEar(Math.random() > 0.5 ? 'left' : 'right');
        }
    };

    const handleScroll = () => {
        // Keep lynx roughly in viewport when scrolling
        if (!state.isBlocking && !state.isMoving) {
            const viewportTop = window.scrollY;
            const viewportBottom = viewportTop + window.innerHeight;
            
            if (state.y < viewportTop || state.y > viewportBottom - 120) {
                moveTo(state.x, viewportTop + 100);
            }
        }
    };

    // ========== Animation Loop ==========
    
    let lastFrame = 0;
    const animate = (timestamp) => {
        if (timestamp - lastFrame > 16) { // ~60fps
            updatePosition();
            swayTail();
            lastFrame = timestamp;
        }
        requestAnimationFrame(animate);
    };

    // ========== Initialize ==========
    
    const init = () => {
        // Create elements
        const container = createLynxContainer();
        
        // Set initial position
        container.style.left = `${state.x}px`;
        container.style.top = `${state.y}px`;
        
        // Start intervals
        state.earTwitchInterval = setInterval(() => {
            if (Math.random() > 0.7) {
                twitchEar(Math.random() > 0.5 ? 'left' : 'right');
            }
        }, 3000);
        
        state.blinkInterval = setInterval(() => {
            if (Math.random() > 0.6) {
                blink();
            }
        }, 4000);
        
        // Event listeners
        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('click', handleClick);
        window.addEventListener('scroll', handleScroll);
        
        // Dismiss button
        document.getElementById('lince-block-dismiss')?.addEventListener('click', dismissBlock);
        
        // Start animation loop
        requestAnimationFrame(animate);
        
        // Start behavior loop (with initial delay)
        setTimeout(pickAction, 3000);
        
        // Initial greeting
        setTimeout(() => {
            showSpeech('*mrrow!*', 2000);
        }, 1500);
        
        console.log('🐱 Lince Spirit awakened!');
    };

    // Wait for DOM
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }

})();
