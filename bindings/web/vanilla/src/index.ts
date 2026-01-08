/**
 * Helion - High-performance data visualization engine
 * Framework-agnostic JavaScript API
 */

export interface ChartOptions {
  width?: number;
  height?: number;
  backgroundColor?: string;
}

export interface ScatterOptions extends ChartOptions {
  x: number[] | Float32Array;
  y: number[] | Float32Array;
  color?: string | string[];
  size?: number | number[];
}

export interface LineOptions extends ChartOptions {
  x: number[] | Float32Array;
  y: number[] | Float32Array;
  color?: string;
  lineWidth?: number;
}

export class Chart {
  private canvas: HTMLCanvasElement;
  
  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
  }
  
  destroy(): void {
    // Cleanup will be implemented
  }
}

export async function scatter(
  canvas: HTMLCanvasElement,
  options: ScatterOptions
): Promise<Chart> {
  // Will be implemented with WASM bindings
  console.log('Scatter plot initialization - coming soon');
  return new Chart(canvas);
}

export async function line(
  canvas: HTMLCanvasElement,
  options: LineOptions
): Promise<Chart> {
  // Will be implemented with WASM bindings
  console.log('Line chart initialization - coming soon');
  return new Chart(canvas);
}

// GPU backend detection
export async function detectGPUBackend(): Promise<'webgpu' | 'webgl' | 'none'> {
  if ('gpu' in navigator) {
    try {
      const adapter = await (navigator as any).gpu.requestAdapter();
      if (adapter) return 'webgpu';
    } catch (e) {
      console.warn('WebGPU not available:', e);
    }
  }
  
  const canvas = document.createElement('canvas');
  const gl = canvas.getContext('webgl2');
  if (gl) return 'webgl';
  
  return 'none';
}

export { ChartOptions, ScatterOptions, LineOptions };
