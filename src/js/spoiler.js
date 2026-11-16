function showSpoiler(id) {
    var spoilerSpan = document.querySelector(`[data-spoiler-id=${id}]`);
    spoilerSpan.classList.remove('spoiler');
}