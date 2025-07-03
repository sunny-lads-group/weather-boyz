import { useState, useEffect } from 'react';
import { fetchUserPolicies } from '../services/policyService';
import type { InsurancePolicy } from '../types';

// Helper function to format policy dates that come as arrays from Rust PrimitiveDateTime
const formatPolicyDate = (dateValue: any): string => {
  try {
    // Check if it's an array format [year, day_of_year, hour, minute, second, nanosecond]
    if (Array.isArray(dateValue) && dateValue.length >= 6) {
      const [year, dayOfYear, hour, minute, second, nanosecond] = dateValue;
      
      // Create date from year and day of year
      const date = new Date(year, 0); // January 1st of the year
      date.setDate(dayOfYear); // Set to the specific day of year
      date.setHours(hour, minute, second, Math.floor(nanosecond / 1000000)); // Convert nanoseconds to milliseconds
      
      return date.toLocaleDateString();
    }
    
    // Check if it's a large timestamp (nanoseconds since epoch)
    const numValue = Number(dateValue);
    if (!isNaN(numValue) && numValue > 1000000000000000000) {
      // Convert nanoseconds to milliseconds
      const dateInMs = numValue / 1000000;
      return new Date(dateInMs).toLocaleDateString();
    }
    
    // Fallback to regular date parsing
    return new Date(dateValue).toLocaleDateString();
  } catch (error) {
    console.error('Error formatting date:', dateValue, error);
    return 'Invalid Date';
  }
};

const MyPolicies = () => {
  const [policies, setPolicies] = useState<InsurancePolicy[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadPolicies = async () => {
      try {
        setLoading(true);
        const userPolicies = await fetchUserPolicies();
        setPolicies(userPolicies);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load policies');
      } finally {
        setLoading(false);
      }
    };

    loadPolicies();
  }, []);

  if (loading) {
    return (
      <div className="flex justify-center items-center min-h-64">
        <div className="text-lg">Loading your policies...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-red-50 border border-red-200 rounded-lg p-4">
        <h2 className="text-lg font-semibold text-red-800 mb-2">Error Loading Policies</h2>
        <p className="text-red-600">{error}</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <h1 className="text-3xl font-bold text-gray-900">Your Weather Insurance Policies</h1>
      
      {policies.length === 0 ? (
        <div className="text-center py-12">
          <div className="text-gray-500 text-lg mb-4">You don't have any insurance policies yet.</div>
          <p className="text-gray-400">Visit the Available Policies page to purchase your first policy.</p>
        </div>
      ) : (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {policies.map((policy) => (
            <div key={policy.id} className="bg-white rounded-lg shadow-md border border-gray-200 p-6">
              <div className="flex justify-between items-start mb-4">
                <h3 className="text-xl font-semibold text-gray-900">{policy.policy_name}</h3>
                <span className={`px-2 py-1 rounded-full text-xs font-medium ${
                  policy.status === 'active' ? 'bg-green-100 text-green-800' :
                  policy.status === 'pending' ? 'bg-yellow-100 text-yellow-800' :
                  'bg-gray-100 text-gray-800'
                }`}>
                  {policy.status || 'Active'}
                </span>
              </div>
              
              <div className="space-y-3">
                <div>
                  <span className="text-sm font-medium text-gray-500">Type:</span>
                  <span className="ml-2 text-sm text-gray-900">{policy.policy_type}</span>
                </div>
                
                <div>
                  <span className="text-sm font-medium text-gray-500">Coverage:</span>
                  <span className="ml-2 text-sm text-gray-900">
                    {policy.currency || '$'}{policy.coverage_amount}
                  </span>
                </div>
                
                <div>
                  <span className="text-sm font-medium text-gray-500">Premium:</span>
                  <span className="ml-2 text-sm text-gray-900">
                    {policy.currency || '$'}{policy.premium_amount}
                  </span>
                </div>
                
                {policy.location_name && (
                  <div>
                    <span className="text-sm font-medium text-gray-500">Location:</span>
                    <span className="ml-2 text-sm text-gray-900">{policy.location_name}</span>
                  </div>
                )}
                
                <div>
                  <span className="text-sm font-medium text-gray-500">Period:</span>
                  <div className="text-sm text-gray-900">
                    {formatPolicyDate(policy.start_date)} - {formatPolicyDate(policy.end_date)}
                  </div>
                </div>
                
                {policy.smart_contract_address && (
                  <div>
                    <span className="text-sm font-medium text-gray-500">Contract:</span>
                    <span className="ml-2 text-xs text-gray-600 font-mono">
                      {policy.smart_contract_address.substring(0, 10)}...
                    </span>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default MyPolicies; 