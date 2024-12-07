import React, { useState } from "react";

const SellBitcoinForm = () => {
  const [cryptocurrency, setCryptocurrency] = useState("Bitcoin");
  const [action, setAction] = useState("sell");
  const currency = ["Bitcoin", "Tether", "Ethereum", "USDC"];
  return (
    <div className="max-w-2xl mt-10 mx-auto p-8 bg-white shadow-lg rounded-xl border border-gray-200">
      <h1 className="text-3xl font-semibold text-gray-800 mb-8 text-center">
        Create an Offer to Sell Bitcoin
      </h1>

      {/* Cryptocurrency Selection */}
      <div className="mb-8">
        <h2 className="text-xl font-medium text-gray-700 mb-4">
          Choose your cryptocurrency
        </h2>
        <div className="flex gap-4 flex-wrap">
          {currency.map((crypto) => (
            <button
              key={crypto}
              onClick={() => setCryptocurrency(crypto)}
              className={`px-6 py-3 rounded-lg text-lg font-medium transition-all duration-300 ${
                cryptocurrency === crypto
                  ? "bg-orange-500 text-white shadow-lg"
                  : "bg-gray-200 text-gray-700 hover:bg-gray-300"
              }`}
            >
              {crypto}
            </button>
          ))}
        </div>
      </div>

      {/* Action Type (Sell/Buy) */}
      <div className="mb-8">
        <h2 className="text-xl font-medium text-gray-700 mb-4">
          What would you like to do?
        </h2>
        <div className="flex gap-6">
          <label className="flex items-center gap-3">
            <input
              type="radio"
              name="action"
              value="sell"
              checked={action === "sell"}
              onChange={() => setAction("sell")}
              className="text-blue-500 focus:ring-2 focus:ring-blue-500"
            />
            <span className="text-gray-700">Sell Bitcoin</span>
          </label>
          <label className="flex items-center gap-3">
            <input
              type="radio"
              name="action"
              value="buy"
              checked={action === "buy"}
              onChange={() => setAction("buy")}
              className="text-blue-500 focus:ring-2 focus:ring-blue-500"
            />
            <span className="text-gray-700">Buy Bitcoin</span>
          </label>
        </div>
        <p className="text-gray-500 text-sm mt-3 text-center">
          {action === "sell"
            ? "Your offer will be listed on the Buy Bitcoin page."
            : "Your offer will be listed on the Sell Bitcoin page."}
        </p>
      </div>

      {/* Payment Method */}
      <div className="mb-8">
        <h2 className="text-xl font-medium text-gray-700 mb-4">
          Payment Method
        </h2>
        <div className="relative">
          <input
            type="text"
            placeholder="Start typing..."
            className="w-full px-6 py-3 border rounded-lg text-gray-700 placeholder-gray-400 focus:ring-2 focus:ring-blue-500"
          />
          <button className="absolute right-4 top-1/2 transform -translate-y-1/2 text-blue-500 hover:underline">
            Show all
          </button>
        </div>
      </div>

      {/* Submit Button */}
      <button className="w-full bg-teal-600 text-white py-3 rounded-lg font-semibold text-lg hover:bg-teal-500 focus:outline-none focus:ring-4 focus:ring-teal-300 transition duration-300">
        Create Offer
      </button>
    </div>
  );
};

export default SellBitcoinForm;
