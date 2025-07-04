import type { PolicyTemplate } from '../../types';

interface PolicyTemplateCardProps {
  template: PolicyTemplate;
  handlePolicyPurchase: (template: PolicyTemplate) => void;
}

const PolicyTemplateCard = ({
  template,
  handlePolicyPurchase,
}: PolicyTemplateCardProps) => {
  const formatCurrency = (amount: string) => {
    const num = parseFloat(amount);
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'ETH',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(num);
  };

  const formatPremiumRate = (rate: string) => {
    const num = parseFloat(rate) * 100;
    return `${num.toFixed(2)}%`;
  };

  const getPolicyTypeIcon = (type: string) => {
    switch (type.toLowerCase()) {
      case 'drought':
        return '☀️';
      case 'rain':
        return '🌧️';
      case 'temperature':
        return '🌡️';
      case 'wind':
        return '💨';
      case 'storm':
        return '⛈️';
      default:
        return '🌤️';
    }
  };

  const getPolicyTypeColor = (type: string) => {
    switch (type.toLowerCase()) {
      case 'drought':
        return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'rain':
        return 'bg-blue-100 text-blue-800 border-blue-200';
      case 'temperature':
        return 'bg-red-100 text-red-800 border-red-200';
      case 'wind':
        return 'bg-gray-100 text-gray-800 border-gray-200';
      case 'storm':
        return 'bg-purple-100 text-purple-800 border-purple-200';
      default:
        return 'bg-green-100 text-green-800 border-green-200';
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-md border border-gray-200 p-6 hover:shadow-lg transition-shadow duration-200 flex flex-col h-full">
      <div className="flex items-start justify-between mb-4">
        <div className="flex items-center space-x-3">
          <span className="text-2xl">
            {getPolicyTypeIcon(template.policy_type)}
          </span>
          <div>
            <h3 className="text-lg font-semibold text-gray-900">
              {template.template_name}
            </h3>
            <span
              className={`inline-flex items-center px-3 py-1 rounded-full text-xs font-medium border ${getPolicyTypeColor(
                template.policy_type
              )}`}
            >
              {template.policy_type.toUpperCase()}
            </span>
          </div>
        </div>
      </div>

      {template.description && (
        <p className="text-gray-600 text-sm mb-4 leading-relaxed">
          {template.description}
        </p>
      )}

      <div className="space-y-3 flex-grow">
        <div className="flex justify-between items-center">
          <span className="text-sm font-medium text-gray-700">
            Coverage Range:
          </span>
          <span className="text-sm text-gray-900">
            {formatCurrency(template.min_coverage_amount)} -{' '}
            {formatCurrency(template.max_coverage_amount)}
          </span>
        </div>

        <div className="flex justify-between items-center">
          <span className="text-sm font-medium text-gray-700">
            Base Premium Rate:
          </span>
          <span className="text-sm text-gray-900 font-medium">
            {formatPremiumRate(template.base_premium_rate)}
          </span>
        </div>
      </div>

      <div className="mt-6 pt-4 border-t border-gray-100">
        <button
          className="w-full bg-orange-500 text-white py-2 px-4 rounded-md text-sm font-medium hover:bg-orange-600 transition-colors duration-200"
          onClick={() => {
            handlePolicyPurchase(template);
          }}
        >
          Purchase Policy
        </button>
      </div>
    </div>
  );
};

export default PolicyTemplateCard;
