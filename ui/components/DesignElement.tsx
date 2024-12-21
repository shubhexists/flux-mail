import React from "react";

interface DesignElementProps {
  className?: string;
}

const DesignElement: React.FC<DesignElementProps> = ({ className }) => {
  return (
    <div className={`design-element ${className}`}>
      <svg
        width="100"
        height="100"
        viewBox="0 0 100 100"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <circle
          cx="50"
          cy="50"
          r="45"
          stroke="var(--accent)"
          strokeWidth="10"
        />
        <path
          d="M50 5L95 50L50 95L5 50L50 5Z"
          fill="var(--secondary)"
          fillOpacity="0.5"
        />
      </svg>
    </div>
  );
};

export default DesignElement;
