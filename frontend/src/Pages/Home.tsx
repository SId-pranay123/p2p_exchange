import Navbar from "@/myComponent/Navbar";
import OfferCard from "@/myComponent/OfferCard";
import { useNavigate } from "react-router-dom";

const Home = () => {
  const navigate = useNavigate();
  // Dummy data to populate the OfferCard
  const offerData = {
    amount: 10,
    created_at: 1629937890, // Timestamp for the date
    crypto_type: "Bitcoin",
    max_trade_limit: 50,
    min_trade_limit: 1,
    payment_methods: "PayPal, Bank Transfer",
    price_per_unit: 45000,
    seller: "JohnDoe123",
    status: "Active", // Could be "Active", "InTrade", "Completed", or "Cancelled"
  };

  // Handle buy action
  const handleBuyClick = () => {
    // navigate to payment page
    navigate("/payment");

    // Here you can implement the logic for the purchase
    // For example: redirect to payment page, update state, etc.
  };

  return (
    <div className="  ">
      <Navbar />
      <OfferCard {...offerData} onBuyClick={handleBuyClick} />
    </div>
  );
};

export default Home;
