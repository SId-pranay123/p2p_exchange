import { DivideIcon } from "lucide-react";
import React from "react";

const OfferCard = ({
  amount,
  created_at,
  crypto_type,
  max_trade_limit,
  min_trade_limit,
  payment_methods,
  price_per_unit,
  seller,
  status,
  onBuyClick,
}) => {
  // Formatting the created_at to a readable date
  const createdAtFormatted = new Date(created_at * 1000).toLocaleDateString();

  return (
    <div className="w-full max-w-4xl mx-auto bg-white rounded-xl shadow-lg border border-gray-300 p-6 mt-6">
      {/* Card Content */}

      <div className="flex flex-col md:flex-row justify-between items-center md:items-start space-y-4 md:space-y-0">
        {/* Seller & Trade Info */}
        <div className="flex flex-col space-y-2 w-full md:w-1/2">
          <span className="font-medium text-lg text-gray-800">
            Buy From - {seller}
          </span>
          <div className="text-sm text-gray-600">
            <p>
              Min Trade Limit:{" "}
              <span className="font-semibold">{min_trade_limit} INR</span>
            </p>
            <p>
              Max Trade Limit:{" "}
              <span className="font-semibold">{max_trade_limit} INR</span>
            </p>
          </div>
        </div>

        {/* Amount & Price Info */}
        <div className="flex flex-col items-center text-sm md:text-base space-y-2 w-full md:w-1/3">
          <span className="text-2xl font-bold text-gray-900">{amount} INR</span>
          <span className="text-xs text-gray-500">
            1 USD = {price_per_unit} USD
          </span>
        </div>
      </div>

      {/* Payment Method */}
      <div className="mt-4 text-gray-800">
        <div className="flex justify-between items-center text-lg">
          <p className="font-medium">Payment Method:</p>
          <p className="font-semibold text-teal-600">{payment_methods}</p>
        </div>
      </div>

      {/* Buy Now Button */}
      <div className="mt-6">
        <button
          className="w-full px-6 py-3 bg-teal-600 text-white rounded-lg text-lg font-semibold hover:bg-teal-500 focus:outline-none focus:ring-2 focus:ring-teal-400 transition duration-300"
          onClick={onBuyClick}
        >
          Buy Now
        </button>
      </div>
    </div>
  );
};

export default OfferCard;
