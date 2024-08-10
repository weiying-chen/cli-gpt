import React from "react";

const Button = ({ children, onClick, className }) => {
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
