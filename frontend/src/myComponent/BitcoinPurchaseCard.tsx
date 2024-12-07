import React from "react";

const BitcoinPurchaseCard = () => {
  return (
    <div className="max-w-5xl mx-auto p-8 bg-white shadow-lg rounded-xl border border-gray-200">
      <h2 className="text-3xl font-semibold text-gray-800 mb-6 text-center">
        How much do you want to Buy?
      </h2>
      <div className="mb-8">
        {/* Input Section */}
        <div className="flex flex-col md:flex-row items-center gap-6 mb-4">
          <div className="flex items-center gap-4 w-full md:w-1/2">
            <label
              htmlFor="amount"
              className="sm:block text-gray-600 font-medium w-[30%] "
            >
              I will PAY
            </label>
            <input
              type="number"
              id="amount"
              placeholder="50000"
              className="border rounded-xl px-6 py-3 w-[80%]  md:w-2/3 text-lg focus:ring-2 focus:ring-teal-400 font-semibold bg-gray-200"
            />
            <span className="text-gray-500 text-lg">INR</span>
          </div>
          <div><p className="font-bold text-xl text-teal-600 ">AND</p></div>
          <div className="flex items-center gap-4 w-full md:w-1/2">
            <label
              htmlFor="receive"
              className="sm:block text-gray-600 font-medium w-[30%]"
            >
               RECEIVE
            </label>
            <input
              type="text"
              id="receive"
              placeholder="0.00548546"
              className="border rounded-xl px-6 py-3 w-[80%]  md:w-2/3  text-lg  cursor-not-allowed focus:ring-2 focus:ring-teal-400  font-semibold bg-gray-200"
              readOnly
            />
            <span className="text-gray-500 text-lg">BTC</span>
          </div>
        </div>
        {/* Information */}
        <p className="text-gray-500 mt-2 text-center">
          You get <strong>46,245.49 INR</strong> worth of Bitcoin
        </p>
      </div>

      {/* Sign up Button */}
      <button className="w-full bg-teal-600 text-white py-4 rounded-md font-semibold text-lg hover:bg-teal-500 focus:outline-none focus:ring-4 focus:ring-teal-300 transition duration-300 mb-6">
        Sign up to buy
      </button>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-8 mt-6">
        {/* About this offer */}
        <div className="bg-gray-50 p-6 rounded-xl shadow-md border border-gray-200">
          <h3 className="text-xl font-semibold text-gray-800 mb-4">
            About this offer
          </h3>
          <p className="text-gray-600 mb-2">
            <span className="font-medium">Seller rate:</span> 9,115,000 INR -
            8.05% above market
          </p>
          <p className="text-gray-600 mb-2">
            <span className="font-medium">Buy limits:</span> Min 50,000 INR -
            Max 974,214 INR
          </p>
          <p className="text-gray-600 mb-2">
            <span className="font-medium">Trade time limit:</span> 30 min
          </p>
          <p className="text-gray-600 mb-2">
            <span className="font-medium">Paxful fee:</span> 0%
          </p>
        </div>
        {/* About the seller */}
        <div className="bg-gray-50 p-6 rounded-xl shadow-md border border-gray-200">
          <h3 className="text-xl font-semibold text-gray-800 mb-4">
            About this seller
          </h3>
          <div className="flex items-center gap-3 mb-4">
            <img
              src="https://flagcdn.com/in.svg"
              alt="India Flag"
              className="w-8 h-8 rounded-full"
            />
            <div className="text-gray-800 font-medium">FOREVER8184</div>
            <span className="bg-green-100 text-green-600 px-3 py-1 rounded-full text-sm font-semibold">
              Power Trader
            </span>
          </div>
          <p className="text-gray-600 mb-2">Positive feedback: 498</p>
          <p className="text-gray-600 mb-2">Negative feedback: 0</p>
          <div className="mt-4">
            <ul className="text-gray-600 list-disc list-inside">
              <li>ID verified</li>
              <li>Email verified</li>
              <li>Address verified</li>
              <li>Phone verified</li>
            </ul>
          </div>
          <p className="text-gray-600 mt-4">
            <span className="font-medium">Average trade speed:</span> 1 min
          </p>
        </div>
      </div>

      {/* Offer Terms */}
      <div className="mt-8 bg-gray-50 p-6 rounded-xl shadow-md border border-gray-200">
        <h3 className="text-xl font-semibold text-gray-800 mb-4">
          Offer terms
        </h3>
        <ul className="text-gray-600 list-disc list-inside space-y-2">
          <li>Only for INDIAN traders (IMPS, GPay, PhonePe, any UPI)</li>
          <li>Buyer needs to share a photo ID (selfie may be asked)</li>
          <li>
            Mobile number for verification (details will be provided after)
          </li>
          <li>No third-party payment accepted</li>
          <li>Payment screenshot also needed</li>
        </ul>
      </div>
    </div>
  );
};

export default BitcoinPurchaseCard;
