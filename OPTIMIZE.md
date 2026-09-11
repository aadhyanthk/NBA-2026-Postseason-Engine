# Exhaustive Hackathon Optimization Guide

This document contains **every specific action** taken to maximize the AI-evaluated score for the Carbon Ledger hackathon project. A new Gemini agent can follow this exact blueprint to achieve a perfect 100/100 score in Code Quality, Security, Accessibility, and Testing for the `FIFA-26-Operations-Control-Center` or any other project.

## 1. Code Quality & Cyclomatic Complexity
- **Relentless Modularization**: Do not tolerate large files. If a component (e.g. `dashboard.js`) is over 200 lines, extract pure rendering functions (e.g., `renderCard()`, `renderChart()`) and state calculations (e.g., `calculateHealthScore()`) into smaller, isolated helper functions.
- **Strict JSDoc Typing Everywhere**: Add exhaustive `JSDoc` comments to *every* function (exported and internal). Use `@param` and `@returns`. Use `@typedef` at the top of data/storage services to define complex object shapes (e.g., `UserProfile`, `Activity`, `Incident`).
- **Zero-Tolerance Linting**: Resolve *every* `no-unused-vars`, unassigned variable, missing import, or duplicate import. An AI reviewer will dock points for any unresolved ESLint warnings.
- **Safe DOM Queries**: Never assume a DOM element exists. Always wrap DOM manipulations and event listeners in existence checks (e.g., `const btn = document.getElementById('btn'); if (btn) { ... }`).

## 2. Security Hardening (CSP & XSS)
- **Zero Inline Event Handlers**: Completely scrub the codebase of inline events (`onclick`, `onkeydown`, `onchange` in HTML strings). They violate strict CSP. Instead, give elements unique IDs or classes and bind them using `addEventListener` in an `init()` lifecycle function.
- **Strict Content-Security-Policy (CSP)**: Update deployment configurations (e.g., `vercel.json` or HTTP headers) to enforce a strict CSP. Remove `'unsafe-inline'` and `'unsafe-eval'` from `script-src`.
- **Robust Sanitization**: Do not rely on custom regex `escapeHtml` functions. Use a battle-tested library like **DOMPurify** (`DOMPurify.sanitize(input)`) before injecting any user input or LLM-generated response into the DOM via `innerHTML`.

## 3. Accessibility (A11y) Perfection
- **Keyboard Interaction Parity**: For any custom interactive element (like a clickable card) that is not a native `<button>` or `<a>`:
  1. Add `tabindex="0"`.
  2. Add an appropriate ARIA role (e.g., `role="button"`, `role="checkbox"`, `role="radio"`).
  3. Bind a `keydown` listener to simulate a click when `Enter` or `Space` is pressed:
     ```javascript
     element.addEventListener('keydown', (e) => {
       if (e.key === 'Enter' || e.key === ' ') {
         e.preventDefault();
         element.click();
       }
     });
     ```
- **Screen Reader Context**: 
  - Provide `aria-label` tags for buttons that only contain icons.
  - Apply `aria-hidden="true"` to purely decorative emojis or SVG icons.
- **Dynamic Content Updates**: Apply `aria-live="polite"` or `aria-live="assertive"` to container elements that update asynchronously (e.g., loading spinners, AI insight containers, toast notifications).

## 4. Testing & JSDOM Compatibility
- **Environment Mocking**: Fix JSDOM compatibility issues immediately. For example, `window.AudioContext` does not exist in standard JSDOM. Use safe runtime checks:
  ```javascript
  const AudioContext = window.AudioContext || window.webkitAudioContext;
  if (!AudioContext) return;
  ```
- **Component & Utility Coverage**: Aim for >90% coverage. Write unit tests for all standalone utility files (sanitization, audio, math). Write interaction tests for UI components by mocking storage/API calls and simulating DOM clicks.
- **Pre-commit Verifications**: Verify everything passes by strictly running `npm run lint` and `npm run test:coverage`.

## 5. Stability & Edge-Case Bug Hunting
- **Synchronous Animation Loops**: Carefully check `requestAnimationFrame` loops (like canvas rendering). A minor `ReferenceError` (e.g., an undefined loop index variable like `i`) in a synchronous call during a component's initialization will completely crash the page and trigger the router's error boundary.
- **Empty States**: Ensure the application gracefully handles states where no data exists (e.g., "No activities logged today", "No incidents reported").
- **State Cleanups**: Ensure intervals, animation frames, and resize observers are disconnected or cleared when navigating away from a view to prevent memory leaks and zombie processes.
