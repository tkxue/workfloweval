const audio = new AudioContext();

await audio.audioWorklet.addModule('player-worklet.js');

const player = new AudioWorkletNode(audio, 'player-worklet');

player.connect(audio.destination);

window.addEventListener('click', () => {
  audio.resume();
  player.port.postMessage({
    type: 'play'
  });
});
