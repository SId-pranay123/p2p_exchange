import { Route, Routes } from "react-router-dom";
// import Home from "./Pages/Home";
// import CreateOffer from "./Pages/CreateOffer";
// import Payment from "./Pages/Payment";
import NotFound from "./Pages/NotFound";
import BitcoinPurchaseCard from "./myComponent/BitcoinPurchaseCard";
import SellBitcoinForm from "./Pages/SellBitcoinForm";
import Home from "./Pages/Home";

function App() {
  return (
    <>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/create-form" element={<SellBitcoinForm />} />
        <Route path="/payment" element={<BitcoinPurchaseCard />} />{" "}
        {/* Wildcard for 404 */}
        <Route path="*" element={<NotFound />} /> {/* Wildcard for 404 */}
      </Routes>
    </>
  );
}

export default App;
