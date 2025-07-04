import { useNotifications } from '../../context/NotificationContext';

const NotificationContainer = () => {
  const { notifications, removeNotification } = useNotifications();

  const getNotificationStyles = (type: string) => {
    switch (type) {
      case 'success':
        return 'bg-green-50 border-green-200 text-green-800';
      case 'error':
        return 'bg-red-50 border-red-200 text-red-800';
      case 'warning':
        return 'bg-yellow-50 border-yellow-200 text-yellow-800';
      case 'info':
        return 'bg-blue-50 border-blue-200 text-blue-800';
      default:
        return 'bg-gray-50 border-gray-200 text-gray-800';
    }
  };

  const getIconForType = (type: string) => {
    switch (type) {
      case 'success':
        return '✓';
      case 'error':
        return '✕';
      case 'warning':
        return '⚠';
      case 'info':
        return 'ℹ';
      default:
        return '';
    }
  };

  if (notifications.length === 0) return null;

  return (
    <div className="fixed top-4 right-4 z-50 space-y-2">
      {notifications.map((notification) => (
        <div
          key={notification.id}
          className={`max-w-lg w-full p-4 border rounded-lg shadow-lg ${getNotificationStyles(notification.type)} transition-all duration-300 ease-in-out`}
        >
          <div className="flex items-start">
            <div className="flex-shrink-0">
              <span className="text-lg font-semibold">
                {getIconForType(notification.type)}
              </span>
            </div>
            <div className="ml-3 flex-1">
              <h3 className="text-sm font-medium">
                {notification.title}
              </h3>
              {notification.message && (
                <p className="mt-1 text-sm opacity-90 break-all overflow-wrap-anywhere">
                  {notification.message}
                </p>
              )}
            </div>
            <div className="ml-4 flex-shrink-0">
              <button
                onClick={() => removeNotification(notification.id)}
                className="inline-flex text-sm opacity-60 hover:opacity-100 transition-opacity cursor-pointer"
              >
                <span className="sr-only">Close</span>
                ✕
              </button>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
};

export default NotificationContainer;