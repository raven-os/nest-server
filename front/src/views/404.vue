<template>
  <div id="app">
    <!--==========================
    App - 404
    ============================-->
    <section id="main404">
        <div class="main404-container">
            <h1>ERROR 404</h1>
            <h2>This page is not available</h2>
            <a href="/">Go back to home page</a>
        </div>
    </section>
  </div>
</template>

<script>
export default {
  data () {
    return {
    }
  },
  computed: {
  },
  beforeMount () {
  },
  methods: {
  }
}
</script>

<style scoped>
#main404 {
    display: table;
    width: 100%;
    height: 100vh;
    background: url(/img/raven404.png) top center fixed;
    background-size: cover;
}

@media (max-width:990px) {
    #main404 {
        background-position: 80% 50%;
    }
}

#main404 .main404-container {
    display: table-cell;
    padding: 10px;
    text-align: center;
    background-size: 100%;
    vertical-align: middle;
}

#main404 .main404-container h1 {
	margin-top: 70px;
	margin-bottom: 200px;
	font-size: 50px;
	color: var(--accent);
}

#main404 .main404-container a {
	color: var(--accent);
}

@media (max-width:990px) {
	#main404 .main404-container h1 {
		margin-top: 170px;
		margin-bottom: 75px;
	}
}
</style>
