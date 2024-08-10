import React from "react";

// Use the function keyword instead of const to define this Button component:
const Button = ({ children, onClick, className, icon }) => {
  function handleClick(event) {
    if (onClick) {
      onClick(event);
    }
  }

  return (
    <button
      className={`${className} button`}
      onClick={handleClick}
      type="submit"
    >
      <span className="icon">{icon}</span>
      {children}
    </button>
  );
};

export default Button;
