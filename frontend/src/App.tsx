import { useState } from 'react'
import './App.css'
import WalletConnect from './components/WalletConnect'

function App() {
  const [connectedAddress, setConnectedAddress] = useState<string>('')
  const [message, setMessage] = useState('')

  const handleWalletConnect = (address: string) => {
    setConnectedAddress(address)
    console.log('Wallet connected:', address)
  }

  return (
    <div className="App">
      <h1>Weather NFT Minter</h1>
      
      <WalletConnect onConnect={handleWalletConnect} />
      
      {connectedAddress && (
        <div className="connected-status">
          <p>Ready to mint your weather NFT!</p>
        </div>
      )}
    </div>
  )
}

export default App