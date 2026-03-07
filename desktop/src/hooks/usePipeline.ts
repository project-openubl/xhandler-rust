import { useState, useCallback } from "react";

export type PipelineStep = "create" | "sign" | "send";

export interface StepOutput {
  step: PipelineStep;
  label: string;
  content: string;
  language: string;
  isError: boolean;
}

export function usePipeline() {
  const [outputs, setOutputs] = useState<StepOutput[]>([]);
  const [currentStep, setCurrentStep] = useState<PipelineStep | null>(null);
  const [loading, setLoading] = useState(false);

  const lastCompletedStep = outputs.length > 0 ? outputs[outputs.length - 1].step : null;

  const canSign = lastCompletedStep === "create" && !outputs.some((o) => o.step === "create" && o.isError);
  const canSend = outputs.some((o) => o.step === "sign" && !o.isError);

  const addOutput = useCallback((output: StepOutput) => {
    setOutputs((prev) => {
      const filtered = prev.filter((o) => o.step !== output.step);
      // Also remove subsequent steps
      const stepOrder: PipelineStep[] = ["create", "sign", "send"];
      const idx = stepOrder.indexOf(output.step);
      const cleaned = filtered.filter((o) => stepOrder.indexOf(o.step) < idx);
      return [...cleaned, output];
    });
    setCurrentStep(output.step);
  }, []);

  const reset = useCallback(() => {
    setOutputs([]);
    setCurrentStep(null);
  }, []);

  const getOutput = useCallback(
    (step: PipelineStep): StepOutput | undefined => {
      return outputs.find((o) => o.step === step);
    },
    [outputs]
  );

  return {
    outputs,
    currentStep,
    setCurrentStep,
    loading,
    setLoading,
    canSign,
    canSend,
    addOutput,
    reset,
    getOutput,
  };
}
