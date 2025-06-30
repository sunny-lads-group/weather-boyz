import { useState, useEffect } from "react";
import LocationInput from "../components/weather/LocationInput";
import PolicyTemplateCard from "../components/policy/PolicyTemplateCard";
import type { PolicyTemplate } from "../types";
import { fetchPolicyTemplates } from "../services/policyService";
import { useNotifications } from "../context/NotificationContext";

const AvailablePolicies = () => {
  const [currentStep, setCurrentStep] = useState(2); // Start on step 2 to show policies directly
  const [policyTemplates, setPolicyTemplates] = useState<PolicyTemplate[]>([]);
  const [loading, setLoading] = useState(false);
  const { addNotification } = useNotifications();

  const steps = [
    { number: 1, title: "Select Location" },
    { number: 2, title: "Select Policy" },
  ];

  useEffect(() => {
    const loadPolicyTemplates = async () => {
      if (currentStep === 2 && policyTemplates.length === 0) {
        setLoading(true);
        try {
          const templates = await fetchPolicyTemplates();
          setPolicyTemplates(templates);
        } catch (error) {
          console.error('Failed to load policy templates:', error);
          addNotification({
            type: 'error',
            title: 'Failed to Load Policies',
            message: 'Unable to fetch available policy templates. Please try again.',
          });
        } finally {
          setLoading(false);
        }
      }
    };

    loadPolicyTemplates();
  }, [currentStep, policyTemplates.length, addNotification]);

  const renderStepContent = () => {
    switch (currentStep) {
      case 1:
        return <LocationInput />;
      case 2:
        return (
          <div className="mt-4">
            <h2 className="text-xl font-semibold mb-6">Available Policy Templates</h2>
            
            {loading ? (
              <div className="flex items-center justify-center py-12">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-orange-500"></div>
                <span className="ml-3 text-gray-600">Loading policy templates...</span>
              </div>
            ) : policyTemplates.length > 0 ? (
              <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-1 xl:grid-cols-2">
                {policyTemplates.map((template) => (
                  <PolicyTemplateCard key={template.id} template={template} />
                ))}
              </div>
            ) : (
              <div className="text-center py-12">
                <p className="text-gray-600 mb-4">No policy templates available at the moment.</p>
                <button
                  onClick={() => window.location.reload()} 
                  className="text-orange-600 hover:text-orange-700 font-medium"
                >
                  Refresh Page
                </button>
              </div>
            )}
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
        <div className={currentStep === 2 ? "max-w-6xl mx-auto" : "max-w-2xl mx-auto"}>
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