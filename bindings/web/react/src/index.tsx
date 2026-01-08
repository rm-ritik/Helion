import React, { useEffect, useRef } from 'react';
import { scatter as helionScatter, line as helionLine, ScatterOptions, LineOptions } from '@helion/vanilla';

export interface ScatterPlotProps extends Omit<ScatterOptions, 'canvas'> {
  className?: string;
  style?: React.CSSProperties;
}

export interface LinePlotProps extends Omit<LineOptions, 'canvas'> {
  className?: string;
  style?: React.CSSProperties;
}

export const ScatterPlot: React.FC<ScatterPlotProps> = ({
  x,
  y,
  color,
  size,
  width = 800,
  height = 600,
  backgroundColor,
  className,
  style,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  
  useEffect(() => {
    if (!canvasRef.current) return;
    
    let chart: any;
    
    helionScatter(canvasRef.current, {
      x,
      y,
      color,
      size,
      width,
      height,
      backgroundColor,
    }).then(c => {
      chart = c;
    });
    
    return () => {
      if (chart) {
        chart.destroy();
      }
    };
  }, [x, y, color, size, width, height, backgroundColor]);
  
  return (
    <canvas
      ref={canvasRef}
      width={width}
      height={height}
      className={className}
      style={style}
    />
  );
};

export const LinePlot: React.FC<LinePlotProps> = ({
  x,
  y,
  color,
  lineWidth,
  width = 800,
  height = 600,
  backgroundColor,
  className,
  style,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  
  useEffect(() => {
    if (!canvasRef.current) return;
    
    let chart: any;
    
    helionLine(canvasRef.current, {
      x,
      y,
      color,
      lineWidth,
      width,
      height,
      backgroundColor,
    }).then(c => {
      chart = c;
    });
    
    return () => {
      if (chart) {
        chart.destroy();
      }
    };
  }, [x, y, color, lineWidth, width, height, backgroundColor]);
  
  return (
    <canvas
      ref={canvasRef}
      width={width}
      height={height}
      className={className}
      style={style}
    />
  );
};
