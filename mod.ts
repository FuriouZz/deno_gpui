export type ColorType =
  | number
  | { r: number; g: number; b: number; a?: number }
  | { h: number; s: number; l: number; a: number };

export interface IDivProps {
  flex?: true;
  justify_center?: true;
  items_center?: true;
  shadow_lg?: true;
  border?: true;
  text_xl?: true;
  bg?: ColorType;
  border_color?: ColorType;
  text_color?: ColorType;
  size?: "auto" | `${number}px` | `${number}rem` | number;
  children?: any[];
}

export function h(tag: any, props: any, ...children: any[]) {
  if (typeof tag === "string") {
    throw new Error(`Tag ${tag} is not supported`);
  }

  if (children.length > 0) {
    props = { ...props, children };
  }

  if (props !== null) {
    return tag(props);
  }

  return tag();
}

export function Div(props: IDivProps = {}) {
  const entries = Object.entries(props).map(([key, value]) => {
    if (/bg|border_color|text_color/.test(key)) {
      if (typeof value === "object" && value !== null) {
        const type = Object.keys(value);
        if (type.includes("r") && type.includes("g") && type.includes("b")) {
          if (type.includes("a")) return [key, { Rgba: value }];
          return [key, { Rgb: value }];
        }
        if (type.includes("h") && type.includes("s") && type.includes("l")) {
          if (type.includes("a")) return { Hsla: value };
          return [key, { Hsl: value }];
        }
      } else {
        return [key, { RgbHex: value }];
      }
    }

    if (key === "children") {
      return [
        key,
        value.map((v: any) => {
          if (typeof v === "string") {
            return { Text: v };
          }
          return v;
        }),
      ];
    }

    if (key === "size") {
      if (value === "auto") {
        return [key, { Auto: true }];
      } else if (typeof value === "string" && value.endsWith("px")) {
        return [key, { DefiniteAbsolutePixels: parseFloat(value) }];
      } else if (typeof value === "string" && value.endsWith("rem")) {
        return [key, { DefiniteAbsoluteRems: parseFloat(value) }];
      } else if (typeof value === "number") {
        return [key, { DefiniteFraction: value }];
      }
    }

    return [key, true];
  });

  const element = {
    Div: Object.fromEntries(entries as any),
  };

  return element;
}

export function render(ui: object) {
  const GPUI_DYLIB_PATH =
    Deno.env.get("GPUI_DYLIB_PATH") ??
    "./ffi_gpui/target/debug/libdeno_gpui.dylib";

  const lib = Deno.dlopen(GPUI_DYLIB_PATH, {
    start: { parameters: ["pointer", "usize"], result: "void" },
  });

  const u8 = new TextEncoder().encode(JSON.stringify(ui));
  const pointer = Deno.UnsafePointer.of(u8);
  lib.symbols.start(pointer, u8.length);
  lib.close();
}
