import { useNavigate } from "react-router-dom";

const Navbar = () => {
  const navigate = useNavigate();
  return (
    <nav className="w-full p-5 bg-gray-800 shadow-md">
      <div className="max-w-screen-xl mx-auto flex justify-between items-center">
        <div className="text-white text-2xl font-semibold">
          <h1>P2P Trading</h1>
        </div>
        <div className="flex space-x-4">
          <button
            className="px-6 py-2 bg-orange-500 text-white rounded-lg hover:opacity-80 transition duration-300 "
            onClick={() => {
              navigate("/create-form");
            }}
          >
            Create Offer
          </button>
          <button className="px-6 py-2 bg-teal-500 text-white rounded-lg hover:opacity-80 transition duration-300">
            Connect Wallet
          </button>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
