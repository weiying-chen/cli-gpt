import React from "react";

const Button = ({ children, onClick, className }) => {
  // Use the function keywoard instead of const in here (no explanation)
  const handleClick = (event) => {
    if (onClick) {
      onClick(event);
    }
  };

  return (
    <button
      className={`${className} button`}
      onClick={handleClick}
      type="button"
    >
      {children}
    </button>
  );
};

export default Button;
