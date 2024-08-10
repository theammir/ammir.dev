const headings = document.querySelectorAll('#slider > h1');
let currentIndex = 0;

function showNextHeading() {
  const currentHeading = headings[currentIndex];
  const nextIndex = (currentIndex + 1) % headings.length;
  const nextHeading = headings[nextIndex];

  // Animate the current heading out to the right
  currentHeading.classList.remove('active');
  currentHeading.classList.add('exit');

  // Remove the 'exit' class from the previous heading
  currentHeading.addEventListener('transitionend', () => {
    currentHeading.classList.remove('exit');
  }, { once: true });

  // Prepare the next heading to enter from the left
  nextHeading.classList.remove('exit');

  // Trigger the entrance animation for the next heading
  setTimeout(() => {
    nextHeading.classList.add('active');
  }, 50); // Small delay to ensure the transition applies correctly

  currentIndex = nextIndex;
}

setInterval(showNextHeading, 4000);
showNextHeading(); // Initial call to display the first heading immediately

document.addEventListener('mousemove', (event) => {
  const { clientX: x, clientY: y } = event;

  // Adjust these values to control the parallax effect
  const parallaxX = x * 0.1;
  const parallaxY = y * 0.1;

  // Apply parallax effect to each background object
  document.querySelectorAll('.parallax').forEach((obj) => {
    obj.style.transform = `translate(${parallaxX}px, ${parallaxY}px)`;
  });
});
