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
  // handle button
  onBuyClick
}) => {
  // Formatting the created_at to a readable date
  const createdAtFormatted = new Date(created_at * 1000).toLocaleDateString();

  return (
    <div className="max-w-4xl mx-auto bg-white rounded-xl shadow-lg border border-gray-200 mt-6 flex">
      
      <div className="p-6 flex-1">
        <h2 className="text-2xl font-semibold text-gray-800">
          {crypto_type} Offer
        </h2>
    
        <div className="mt-4 space-y-4">
          {/* Seller */}
          <div className="flex justify-between">
            <span className="text-gray-600">Seller:</span>
            <span className="font-medium text-gray-800">{seller}</span>
          </div>

          {/* Amount */}
          <div className="flex justify-between">
            <span className="text-gray-600">Amount:</span>
            <span className="font-medium text-gray-800">{amount}</span>
          </div>

          {/* Price per Unit */}
          <div className="flex justify-between">
            <span className="text-gray-600">Price per Unit:</span>
            <span className="font-medium text-gray-800">{price_per_unit}</span>
          </div>

          {/* Min Trade Limit */}
          <div className="flex justify-between">
            <span className="text-gray-600">Min Trade Limit:</span>
            <span className="font-medium text-gray-800">{min_trade_limit}</span>
          </div>

          {/* Max Trade Limit */}
          <div className="flex justify-between">
            <span className="text-gray-600">Max Trade Limit:</span>
            <span className="font-medium text-gray-800">{max_trade_limit}</span>
          </div>

          {/* Payment Methods */}
          <div className="flex justify-between">
            <span className="text-gray-600">Payment Methods:</span>
            <span className="font-medium text-gray-800">{payment_methods}</span>
          </div>

          {/* Created At */}
          <div className="flex justify-between">
            <span className="text-gray-600">Created At:</span>
            <span className="font-medium text-gray-800">
              {createdAtFormatted}
            </span>
          </div>

          {/* Status */}
          <div className="flex justify-between">
            <span className="text-gray-600">Status:</span>
            <span
              className={`font-medium text-white py-1 px-3 rounded-lg 
                ${
                  status === "Active"
                    ? "bg-green-500"
                    : status === "InTrade"
                    ? "bg-yellow-500"
                    : status === "Completed"
                    ? "bg-blue-500"
                    : "bg-red-500"
                }`}
            >
              {status}
            </span>
          </div>
        </div>
      </div>

      {/* Optional Image or Thumbnail (if needed) */}
      <div className="flex-shrink-0 w-48 h-48 bg-gray-100 rounded-xl mx-4">
        {/* Here you can add an image if required */}
        {/* <img src="path/to/your/image.jpg" alt="Offer Image" className="object-cover w-full h-full rounded-xl" /> */}
      </div>
    </div>
  );
};

export default OfferCard;
