// Editable lobe geometry from the Cerebri brain keyframes. These are visual
// concepts, not Rust module boundaries or a second planning model.
export const brainModules = [
  {
    "id": "scope",
    "label": "Scope",
    "side": -1,
    "path": "M-14-219 C-30-259-90-269-132-247 C-179-256-220-220-221-175 C-254-148-254-96-226-65 L-176-73 C-149-103-139-127-108-132 L-31-149 Z",
    "folds": [
      "M-38-222 C-77-249-102-219-109-185 C-141-202-180-186-184-145",
      "M-52-182 C-88-185-83-143-121-157",
      "M-203-111 C-205-149-232-135-232-111"
    ],
    "dx": -175,
    "dy": -92,
    "angle": -9,
    "cx": -122,
    "cy": -167,
    "color": "#42d9ff"
  },
  {
    "id": "known-state",
    "label": "Known State",
    "side": -1,
    "path": "M-230-52 C-263-30-272 18-247 55 C-254 99-219 137-184 118 L-140 79 L-115 21 L-131-34 L-172-57 Z",
    "folds": [
      "M-232-22 C-208-42-167-24-180 14 C-204 16-222 40-207 70",
      "M-151 17 C-136 61-187 52-187 91"
    ],
    "dx": -235,
    "dy": 24,
    "angle": -13,
    "cx": -200,
    "cy": 35,
    "color": "#65b7ff"
  },
  {
    "id": "constraints",
    "label": "Hard Constraints",
    "side": -1,
    "path": "M-174 131 L-127 88 L-75 96 L-62 146 L-69 201 C-89 241-145 235-164 205 C-200 194-211 153-174 131 Z",
    "folds": [
      "M-151 141 L-124 117 L-95 117 L-90 149 L-117 168 L-117 194",
      "M-170 173 L-147 192 L-142 213"
    ],
    "dx": -163,
    "dy": 132,
    "angle": 10,
    "cx": -130,
    "cy": 165,
    "color": "#f38dac"
  },
  {
    "id": "candidates",
    "label": "Candidate Space",
    "side": 1,
    "path": "M14-219 C30-259 90-269 132-247 C179-256 220-220 221-175 C254-148 254-96 226-65 L176-73 C149-103 139-127 108-132 L31-149 Z",
    "folds": [
      "M38-222 C77-249 102-219 109-185 C141-202 180-186 184-145",
      "M52-182 C88-185 83-143 121-157",
      "M203-111 C205-149 232-135 232-111"
    ],
    "dx": 175,
    "dy": -92,
    "angle": 9,
    "cx": 122,
    "cy": -167,
    "color": "#66e9e4"
  },
  {
    "id": "preferences",
    "label": "Preferences",
    "side": 1,
    "path": "M230-52 C263-30 272 18 247 55 C254 99 219 137 184 118 L140 79 L115 21 L131-34 L172-57 Z",
    "folds": [
      "M232-22 C208-42 167-24 180 14 C204 16 222 40 207 70",
      "M151 17 C136 61 187 52 187 91"
    ],
    "dx": 235,
    "dy": 24,
    "angle": 13,
    "cx": 200,
    "cy": 35,
    "color": "#b48bff"
  },
  {
    "id": "validity",
    "label": "Validity",
    "side": 1,
    "path": "M174 131 L127 88 L75 96 L62 146 L69 201 C89 241 145 235 164 205 C200 194 211 153 174 131 Z",
    "folds": [
      "M151 141 L124 117 L95 117 L90 149 L117 168 L117 194",
      "M170 173 L147 192 L142 213"
    ],
    "dx": 163,
    "dy": 132,
    "angle": -10,
    "cx": 130,
    "cy": 165,
    "color": "#66e9c0"
  },
  {
    "id": "ordering",
    "label": "Deterministic Ordering",
    "side": 0,
    "path": "M0-129 L48-91 L64-22 L51 63 L25 114 L-25 114 L-51 63 L-64-22 L-48-91 Z",
    "folds": [
      "M-31-67 L-31 44 L-17 76 M0-91 L0 91 M31-67 L31 44 L17 76"
    ],
    "dx": 0,
    "dy": 0,
    "angle": 0,
    "cx": 0,
    "cy": 0,
    "color": "#4ba7ff"
  },
  {
    "id": "proposal",
    "label": "Proposal",
    "side": 0,
    "path": "M-27 127 L27 127 L36 169 L20 240 L0 258 L-20 240 L-36 169 Z",
    "folds": [
      "M-13 148 L-13 221 L0 237 L13 221 L13 148"
    ],
    "dx": 0,
    "dy": 150,
    "angle": 0,
    "cx": 0,
    "cy": 187,
    "color": "#3fe9ef"
  }
] as const;

export const clamp = (value: number) => Math.max(0, Math.min(1, value));
export const mix = (from: number, to: number, amount: number) => from + (to - from) * amount;
export const ramp = (progress: number, start: number, end: number) => {
  const t = clamp((progress - start) / (end - start));
  return t * t * (3 - 2 * t);
};

// Button stops are navigation aids. Every transform is sampled from the continuous
// scroll position; no timers or rounded scene index drive the exploded geometry.
export const sceneStops = [0, .4, .85, .875, .915, .96, 1];
export const sceneIndex = (progress: number) => progress < .12 ? 0 : progress < .83 ? 1 : progress < .86 ? 2 : progress < .89 ? 3 : progress < .935 ? 4 : progress < .98 ? 5 : 6;
export const focusWindows: Record<string, [number, number]> = {
  scope: [.44, .5], 'known-state': [.5, .55], constraints: [.55, .6],
  candidates: [.6, .66], validity: [.66, .71], ordering: [.71, .76],
  proposal: [.76, .81], authority: [.81, .86]
};
export const focusStrength = (progress: number, id: string) => {
  const range = focusWindows[id];
  return range ? ramp(progress, range[0] - .025, range[0]) * (1 - ramp(progress, range[1], range[1] + .025)) : 0;
};

