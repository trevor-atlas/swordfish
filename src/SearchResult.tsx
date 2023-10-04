interface ResultProps {
  heading: string;
  subtext: string;
}

export default function Result({ heading, subtext }: ResultProps) {
  return (
    <li>
      <span class="result-heading">{heading}</span>
      <span class="result-subtext">{subtext}</span>
    </li>
  );
}
