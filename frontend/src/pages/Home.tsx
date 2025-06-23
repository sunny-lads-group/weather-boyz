import { useWallet } from '../context/WalletContext';

const Home = () => {
  const { isConnected } = useWallet();

  return (
    <div className="max-w-4xl mx-auto text-center">
      <h1 className="text-4xl font-bold text-gray-800 mb-6">
        BUY PARAMETRIC WEATHER INSURANCE NOW!
      </h1>
      
      <p className="text-xl text-gray-600 mb-8">
        Protect yourself against weather-related risks with our innovative insurance solutions.
      </p>

      {!isConnected && (
        <div className="bg-orange-100 p-6 rounded-lg">
          <p className="text-orange-800">
            Please connect your wallet to access available policies.
          </p>
        </div>
      )}

    </div>
  );
};

export default Home;