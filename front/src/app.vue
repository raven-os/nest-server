<template>
  <div id="full-container">
    <!--==========================
    Header
    ============================-->
    <header id="header">
      <div class="container">
        <b-navbar
          class="app-navbar"
          toggleable="md"
          type="dark">
          <b-navbar-toggle target="nav_collapse" />
          <b-navbar-brand to="/"><img src="/img/raven_title.png"></b-navbar-brand>
          <b-collapse
            id="nav_collapse"
            is-nav/>
        </b-navbar>
        <!-- #nav-menu-container -->
      </div>
    </header>
    <!-- #header -->

    <div class="content">
      <router-view class="view" :key="$route.fullPath"/>
    </div>

    <!--==========================
    Footer
    ============================-->
    <footer id="footer">
      <div class="footer-top">
        <div class="container"/>
      </div>
      <div class="container">
        <div class="copyright">&copy; Copyright <strong>Raven</strong>. All Rights Reserved</div>
      </div>
    </footer>
  </div>
</template>

<script>
export default {
  data () {
    return { activeItem: 'home' }
  },
  methods: {
    isActive: function (menuItem) {
      return this.activeItem === menuItem
    },
    setActive: function (menuItem) {
      this.activeItem = menuItem
    }
  }
}
</script>

<style scoped>
#full-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.content {
  flex: 1 0 auto;
  padding: 0px;
}

/* -----------------------------------
Header
----------------------------------- */
#header {
  padding: 10px 0;
  position: fixed;
  left: 0;
  top: 0;
  right: 0;
  transition: all 0.5s;
  z-index: 997;
  background: var(--primary-dark);
}

.app-navbar {
  background-color: var(--primary-dark);
  text-transform: uppercase;
}

.app-footer {
  background-color: var(--primary-dark);
  font-size: 12px;
  text-align: center;
}

.nav-item {
  color: var(--white) !important;
}

.nav-item-active {
  color: var(--accent) !important;
}

/* -----------------------------------
Footer
----------------------------------- */
#footer {
  background: var(--primary-dark);
  padding: 30px 0;
  color: var(--white);
  font-size: 14px;
  width: 100%;
  height: 80px;
}

#footer .copyright {
  text-align: center;
}
</style>
