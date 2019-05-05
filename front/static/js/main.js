/*
** Searchbar
*/

/* Update the selected element in the "Search by" list */
$('.searchbar .dropdown-menu a').click(function(){
    $('.searchbar .dropdown-menu').children().removeClass('active').removeClass('bg-accent');
    $(this).addClass('active').addClass('bg-accent');
});

/* Process the search */
function processSearch() {
    let text = $('.searchbar input').val();
    let search_by = $('.searchbar .dropdown-menu .active').text();

    window.location="/search?q=" + encodeURIComponent(text) + "&search_by=" + encodeURIComponent(search_by.toLowerCase());
}

/* Process the search when the search button is clicked */
$('.searchbar-search').click(function(){
    processSearch();
});

/* Process the search when the form is submitted */
$('.searchbar').on('submit', function(){
    processSearch();
    return false;
});

/*
** Return the GET parameters
** Source: https://stackoverflow.com/questions/5448545/
*/
function findGetParameter(parameterName) {
    var result = null;

    location.search
        .substr(1)
        .split("&")
        .forEach(function (item) {
            var tmp = item.split("=");
            if (tmp[0] === parameterName) {
                result = decodeURIComponent(tmp[1]);
            }
        })
    ;

    return result;
}

/* Set the default search option */
$(document).ready(function(){
    var search_by = findGetParameter('search_by');

    if (search_by !== null) {
        $('.dropdown-item.search-' + search_by).addClass('active').addClass('bg-accent');
    } else {
        $('.dropdown-item.search-name').addClass('active').addClass('bg-accent');
    }
    console.log("yay + " + search_by);
})
