import { useState, useEffect, use } from 'react';
import { useNavigate } from 'react-router-dom';
import LocationInput from '../components/weather/LocationInput';
import type { LocationData } from '../components/weather/LocationInput';
import PolicyTemplateCard from '../components/policy/PolicyTemplateCard';
import type { PolicyTemplate } from '../types';
import { fetchPolicyTemplates } from '../services/policyService';
import { useWallet } from '../context/WalletContext';
import { useNotifications } from '../context/NotificationContext';
import { BrowserProvider, parseEther } from 'ethers';
import { getContract } from '../utils/contract';

const AvailablePolicies = () => {
  const [currentStep, setCurrentStep] = useState(1); // Start on step 1 for location selection
  const [policyTemplates, setPolicyTemplates] = useState<PolicyTemplate[]>([]);
  const [loading, setLoading] = useState(false);
  const [locationData, setLocationData] = useState<LocationData | null>(null);
  const wallet = useWallet();
  const navigate = useNavigate();
  const { addNotification } = useNotifications();

  const steps = [
    { number: 1, title: 'Select Location' },
    { number: 2, title: 'Select Policy' },
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
        } finally {
          setLoading(false);
        }
      }
    };

    loadPolicyTemplates();
  }, [currentStep, policyTemplates.length]);

  const handleLocationSelect = (data: LocationData) => {
    console.log('Location data received:', data);
    setLocationData(data);
  };

  const handlePolicyPurchase = async (template: PolicyTemplate) => {
    if (!window.ethereum || !wallet) {
      addNotification({
        type: 'warning',
        title: 'Wallet Required',
        message: 'Please connect your wallet first to purchase a policy.',
        duration: 5000
      });
      return;
    }

    try {
      const provider = new BrowserProvider(window.ethereum);
      const contract = await getContract(provider);

      // Example values - adjust these based on your template data
      const duration = 30 * 24 * 60 * 60; // 30 days in seconds
      const payout = parseEther(template.max_coverage_amount); // Convert to Wei
      const threshold = -5; // Example threshold temperature
      const eventType = 'TEMP_BELOW';
      const h3HexId = '8928308280fffff'; // Example H3 hex ID

      // Calculate premium (10% of payout as per contract requirement)
      const premium = payout / BigInt(10);

      // Create transaction
      const tx = await contract.buyPolicy(
        duration,
        payout,
        threshold,
        eventType,
        h3HexId,
        { value: premium }
      );

      // Wait for transaction to be mined
      await tx.wait();

      addNotification({
        type: 'success',
        title: 'Policy Purchased Successfully!',
        message: `Your weather insurance policy has been purchased. Transaction hash: ${tx.hash}`,
        duration: 10000
      });

      // Redirect to home page after successful purchase
      navigate('/');
      
      // TODO: Call the backend to create a policy
    } catch (error) {
      console.error('Error purchasing policy:', error);
      addNotification({
        type: 'error',
        title: 'Purchase Failed',
        message: 'Failed to purchase policy. Please try again.',
        duration: 5000
      });
    }
  };

  const renderStepContent = () => {
    switch (currentStep) {
      case 1:
        return <LocationInput onLocationSelect={handleLocationSelect} />;
      case 2:
        return (
          <div className="mt-4">
            <h2 className="text-xl font-semibold mb-6">
              Available Policy Templates
            </h2>

            {locationData && (
              <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-6">
                <h3 className="text-sm font-medium text-blue-800 mb-2">
                  Selected Location
                </h3>
                <div className="text-sm text-blue-700">
                  <p>
                    <strong>Coordinates:</strong> {locationData.latitude},{' '}
                    {locationData.longitude}
                  </p>
                  {locationData.h3Index && (
                    <p>
                      <strong>H3 Index:</strong> {locationData.h3Index}
                    </p>
                  )}
                  {locationData.weatherData && (
                    <div className="mt-2 grid grid-cols-2 md:grid-cols-5 gap-2 text-xs">
                      <span>🌡️ {locationData.weatherData.temperature}°C</span>
                      <span>💧 {locationData.weatherData.humidity}%</span>
                      <span>💨 {locationData.weatherData.wind_speed} m/s</span>
                      <span>
                        🌧️ {locationData.weatherData.precipitation} mm
                      </span>
                      <span>
                        🌡️ Feels{' '}
                        {locationData.weatherData.feels_like.toFixed(0)}°C
                      </span>
                    </div>
                  )}
                </div>
              </div>
            )}

            {loading ? (
              <div className="flex items-center justify-center py-12">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-orange-500"></div>
                <span className="ml-3 text-gray-600">
                  Loading policy templates...
                </span>
              </div>
            ) : policyTemplates.length > 0 ? (
              <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-1 xl:grid-cols-2">
                {policyTemplates.map((template) => (
                  <PolicyTemplateCard
                    key={template.id}
                    template={template}
                    handlePolicyPurchase={handlePolicyPurchase}
                  />
                ))}
              </div>
            ) : (
              <div className="text-center py-12">
                <p className="text-gray-600 mb-4">
                  No policy templates available at the moment.
                </p>
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
                      ? 'border-blue-500 bg-blue-500 text-white'
                      : 'border-gray-300 text-gray-300'
                  }`}
              >
                {step.number}
              </div>

              <div className="ml-2">
                <div
                  className={`text-sm font-medium ${
                    currentStep >= step.number
                      ? 'text-blue-500'
                      : 'text-gray-500'
                  }`}
                >
                  {step.title}
                </div>
              </div>

              {index < steps.length - 1 && (
                <div
                  className={`w-24 h-1 mx-4 ${
                    currentStep > step.number ? 'bg-blue-500' : 'bg-gray-300'
                  }`}
                />
              )}
            </div>
          ))}
        </div>
      </div>

      <div className="bg-white rounded-lg shadow-lg p-6">
        <div
          className={
            currentStep === 2 ? 'max-w-6xl mx-auto' : 'max-w-2xl mx-auto'
          }
        >
          {renderStepContent()}
        </div>
      </div>

      <div className="flex justify-between mt-6">
        <button
          onClick={() => setCurrentStep(Math.max(1, currentStep - 1))}
          disabled={currentStep === 1}
          className={`px-4 py-2 rounded ${
            currentStep === 1
              ? 'bg-gray-300 cursor-not-allowed'
              : 'bg-blue-500 text-white hover:bg-blue-600'
          }`}
        >
          Previous
        </button>
        <button
          onClick={() => setCurrentStep(Math.min(2, currentStep + 1))}
          disabled={currentStep === 2 || (currentStep === 1 && !locationData)}
          className={`px-4 py-2 rounded ${
            currentStep === 2 || (currentStep === 1 && !locationData)
              ? 'bg-gray-300 cursor-not-allowed'
              : 'bg-blue-500 text-white hover:bg-blue-600'
          }`}
        >
          {currentStep === 1 && !locationData
            ? 'Select Location First'
            : 'Next'}
        </button>
      </div>
    </div>
  );
};

export default AvailablePolicies;
