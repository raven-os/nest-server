<template>
  <div id="app">
    <!--==========================
    App - Search
    ============================-->
    <section id="packages-search">
      <b-container>
        <h1>Packages - Stable</h1>
        <div class="search-zone">
          <b-row>
            <b-col/>
            <b-col
              cols="12"
              md="10">
              <b-input-group
                class="search-input-group">
                <input
                  ref="search"
                  v-model="query"
                  class="form-control search-input"
                  type="text"
                  placeholder="Search"
                  @keyup.enter="updateURL">
                <select
                  slot="prepend"
                  v-model="field"
                  class="search-select">
                  <option value="repository">Repository</option>
                  <option value="category">Category</option>
                  <option value="name">Name</option>
                  <option value="version">Version</option>
                  <option value="description">Description</option>
                  <option value="tags">Tags</option>
                  <option value="created_at">Creation date</option>
                </select>
                <b-button
                  slot="append"
                  class="search-button"
                  @click="updateURL">
                  <i class="fas fa-search"/>
                </b-button>
              </b-input-group>
            </b-col>
            <b-col/>
          </b-row>
        </div>
      </b-container>
    </section>

    <!--==========================
    App - List
    ============================-->
    <section id="packages-list">
      <b-container>
        <b-table
          id="packages-table"
          :items="packagesProvider"
          :fields="fields"
          :sort-by.sync="sortBy"
          :sort-desc.sync="sortDesc"
          :busy="isBusy"
          tbody-tr-class="table-row"
          thead-class="list-thead"
          responsive
          striped
          no-local-sorting
          show-empty
          empty-text="There is no package available"
          @row-clicked="onRowClick"
          @sort-changed="onSortChanged" />
      </b-container>
    </section>
  </div>
</template>

<script>
export default {
  data () {
    return {
      query: '',
      field: 'name',
      results: {
        packages: { error: null, data: [] }
      },
      isBusy: false,
      sortBy: 'created_at',
      sortDesc: true,
      fields: [
        { key: 'name',
          label: 'Package name',
          sortable: true,
          class: 'td-name',
          tdClass: 'list-table-cell'
        },
        { key: 'category',
          label: 'Category',
          sortable: true,
          class: 'td-cat',
          tdClass: 'list-table-cell'
        },
        { key: 'version',
          label: 'Version',
          sortable: true,
          class: 'td-ver',
          tdClass: 'list-table-cell'
        },
        { key: 'description',
          label: 'Description',
          sortable: true,
          class: 'td-desc',
          tdClass: 'list-table-cell'
        },
        { key: 'created_at',
          label: 'Creation date',
          sortable: true,
          class: 'td-date',
          tdClass: 'list-table-cell',
          formatter: (value) => {
            return moment((value)).fromNow()
          }
        }
      ]
    }
  },
  computed: {
    errorPackage () {
      return this.results.packages.error
    },
    packages () {
      return this.results.packages.data
    },
    direction () {
      return this.sortDesc ? 'desc' : 'asc'
    }
  },
  beforeMount () {
    this.updateTable()
  },
  methods: {
    updateURL () {
      var data = {}
      data['sort_by'] = this.sortBy
      data['order_by'] = this.direction
      if (this.query) {
        data[this.field] = this.query
      }
      this.$router.push({ path: '', query: data })
      this.updateTable()
    },
    updateTable () {
      this.$root.$emit('bv::refresh::table', 'packages-table')
    },
    resetQuery () {
      this.sortBy = 'created_at'
      this.sortDesc = true
      this.field = 'name'
      this.query = ''
      this.updateURL()
    },
    packagesProvider (ctx) {
      this.isBusy = true
      let url = process.env.VUE_APP_API + '/search'
      var query = this.$route.query
      if (Object.keys(query).length === 0) {
        this.resetQuery()
        return
      } else {
        this.sortBy = query['sort_by']
        this.sortDesc = this.directionBoolean(query['order_by'])
        if (Object.keys(query).length === 3) {
          this.field = Object.keys(query)[2]
          this.query = query[this.field]
        }
      }
      return this.$http.post(url, query, { emulateJSON: true }).then(
        response => {
          this.results.packages.data = response.body
          let items = []
          var data = this.results.packages.data
          for (var i = 0; i < data.length; ++i) {
            var dataItem = data[i]['metadata']
            var newItem = {
              name: dataItem['name'],
              category: dataItem['category'],
              version: dataItem['version'],
              description: dataItem['description'],
              created_at: dataItem['created_at']
            }
            items.push(newItem)
          }
          this.isBusy = false
          return items
        },
        response => {
          this.results.packages.error = response.body.error_description
          return []
        }
      )
    },
    onRowClick (item, index, event) {
      this.$router.push({ name: 'Details',
        params: {
          name: item.name,
          version: item.version,
          category: item.category }})
    },
    onSortChanged (ctx) {
      if (ctx.sortBy !== this.sortBy) {
        this.sortDesc = false
      } else {
        this.sortDesc = this.direction === 'asc'
      }
      this.sortBy = ctx.sortBy
      this.updateURL()
    },
    directionBoolean (dir) {
      return dir === 'desc'
    }
  }
}
</script>

<style scoped>
/* PACKAGES-SEARCH
----------------------------------- */
#packages-search {
  margin-top: 200px;
}

#packages-search h1 {
  text-align: center;
  font-size: 45px;
  margin-top: 50px;
}

#packages-search .search-zone {
  padding: 10px;
  margin-top: 30px;
}

#packages-search .search-input-group {
  margin: 0 auto;
}

#packages-search .search-input {
  font-family: sans-serif;
  font-weight: 500;
  font-size: 16px;
  display: inline-block;
  padding: 8px 28px;
  border-width: 1px 1px 1px 1px;
  border-style: solid;
  border-color: var(--primary-dark);
  color: var(--primary-dark);
  background: rgba(247, 244, 248, 0.7);
  height: 50px;
}

#packages-search .search-input:focus {
  box-shadow: inset 0 1px 1px rgba(0, 0, 0, 0.075), 0 0 8px rgba(0, 0, 0, 0.6);
}

#packages-search .search-select {
  font-family: sans-serif;
  font-size: 16px;
  display: inline-block;
  padding-left: 20px;
  border-width: 1px 0px 1px 1px;
  border-style: solid;
  border-color: var(--primary-dark);
  color: var(--white);
  border-radius: 5px 0px 0px 5px;
  height: 50px;
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  background: var(--accent)
    url("https://cdn3.iconfinder.com/data/icons/google-material-design-icons/48/ic_keyboard_arrow_down_48px-128.png")
    no-repeat;
  background-size: 20px;
  background-position: right 10px center;
  /*width: 100%;*/
  width: 175px;
}

#packages-search .search-button {
  font-family: sans-serif;
  font-size: 16px;
  display: inline-block;
  border-width: 1px 1px 1px 1px;
  border-style: solid;
  border-color: var(--primary-dark);
  color: var(--white);
  border-radius: 0px 5px 5px 0px;
  height: 50px;
  background: rgba(237, 37, 78, 0.8);
}

#packages-search .search-button.active.focus,
#packages-search .search-button.active:focus,
#packages-search .search-button.focus,
#packages-search .search-button.focus:active,
#packages-search .search-button:active:focus,
#packages-search .search-button:focus {
  outline: 0;
  outline-offset: 0;
  background-image: none;
  -webkit-box-shadow: none;
  box-shadow: none;
}

#packages-search .search-button:hover {
  background: var(--accent);
  color: var(--primary);
  cursor: pointer;
}

/* PACKAGES-LIST
----------------------------------- */
#packages-list {
  margin-top: 120px;
}

#packages-list .packages-list-error {
  text-align: center;
  margin-top: 50px;
  font-weight: bold;
}
</style>
