// scripts/deploy.js

const hre = require('hardhat');

async function main() {
  const [deployer] = await hre.ethers.getSigners();

  console.log('Deploying contracts with:', deployer.address);

  const Insurance = await hre.ethers.getContractFactory('WeatherInsurance');

  const insurance = await Insurance.deploy();

  await insurance.waitForDeployment();

  console.log('WeatherInsurance deployed to:', await insurance.getAddress());
}

main().catch((error) => {
  console.error('Deployment failed:', error);
  process.exitCode = 1;
});
