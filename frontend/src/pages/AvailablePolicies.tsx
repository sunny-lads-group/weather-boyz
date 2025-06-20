import { useState } from "react";
import LocationInput from "../components/weather/LocationInput";

const AvailablePolicies = () => {
  const [currentStep, setCurrentStep] = useState(1);

  const steps = [
    { number: 1, title: "Select Location" },
    { number: 2, title: "Select Policy" },
  ];

  const renderStepContent = () => {
    switch (currentStep) {
      case 1:
        return <LocationInput onComplete={() => setCurrentStep(2)} />;
      case 2:
        return (
          <div className="mt-4">
            <h2 className="text-xl font-semibold mb-4">Available Policies</h2>
            <p>Policy selection content goes here...</p>
          </div>
        );
      default:
        return null;
    }
  };

  return (
    <div className="max-w-4xl mx-auto p-6">
      <h1 className="text-2xl font-bold text-center mb-8">
        Weather Insurance Policies
      </h1>

      <div className="mb-8">
        <div className="flex items-center justify-center">
          {steps.map((step, index) => (
            <div key={step.number} className="flex items-center">
              <div
                className={`flex items-center justify-center w-10 h-10 rounded-full border-2 
                  ${
                    currentStep >= step.number
                      ? "border-blue-500 bg-blue-500 text-white"
                      : "border-gray-300 text-gray-300"
                  }`}
              >
                {step.number}
              </div>

              <div className="ml-2">
                <div
                  className={`text-sm font-medium ${
                    currentStep >= step.number
                      ? "text-blue-500"
                      : "text-gray-500"
                  }`}
                >
                  {step.title}
                </div>
              </div>

              {index < steps.length - 1 && (
                <div
                  className={`w-24 h-1 mx-4 ${
                    currentStep > step.number
                      ? "bg-blue-500"
                      : "bg-gray-300"
                  }`}
                />
              )}
            </div>
          ))}
        </div>
      </div>

      <div className="bg-white rounded-lg shadow-lg p-6">
        <div className="max-w-2xl mx-auto">
          {renderStepContent()}
        </div>
      </div>

      <div className="flex justify-between mt-6">
        <button
          onClick={() => setCurrentStep(Math.max(1, currentStep - 1))}
          disabled={currentStep === 1}
          className={`px-4 py-2 rounded ${
            currentStep === 1
              ? "bg-gray-300 cursor-not-allowed"
              : "bg-blue-500 text-white hover:bg-blue-600"
          }`}
        >
          Previous
        </button>
        <button
          onClick={() => setCurrentStep(Math.min(2, currentStep + 1))}
          disabled={currentStep === 2}
          className={`px-4 py-2 rounded ${
            currentStep === 2
              ? "bg-gray-300 cursor-not-allowed"
              : "bg-blue-500 text-white hover:bg-blue-600"
          }`}
        >
          Next
        </button>
      </div>
    </div>
  );
};

export default AvailablePolicies;