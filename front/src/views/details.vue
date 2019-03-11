<template>
  <div id="app">
    <b-container class="top-container">
      <b-row>
        <b-col>
          <div class="package-name">{{ category }}/<span style="color: var(--accent)">{{ name }}</span></div>
          <div
            v-if="getTagsNb > 0"
            class="tags-space">
            <b-badge
              v-for="item in getTags()"
              :key="item"
              class="tag-badge"
              variant="dark">
              {{ item }}
            </b-badge>
          </div>
        </b-col>
      </b-row>
    </b-container>
    <b-container class="mid-container">
      <b-table
        id="version-table"
        :items="detailsProvider"
        :fields="versionFields"
        tbody-tr-class="table-row-nohover"
        thead-class="list-thead"
        responsive />
      <b-table
        id="metadata-table"
        :items="metadataProvider"
        :fields="metaFields"
        tbody-tr-class="table-row-nohover"
        thead-class="list-thead"
        responsive
        striped />
      <b-table
        id="misc-table"
        :items="miscProvider"
        :fields="miscFields"
        tbody-tr-class="table-row-nohover"
        thead-class="list-thead"
        responsive
        striped />
      <b-table
        id="dependencies-table"
        :items="depProvider"
        :fields="depFields"
        tbody-tr-class="table-row-nohover"
        thead-class="list-thead"
        responsive
        striped
        show-empty
        empty-text="No dependency for this package" />
    </b-container>
  </div>
</template>

<script>
export default {
  data () {
    return {
      results: { error: null },
      category: '',
      name: '',
      version: '',
      description: '',
      tags: '',
      date: '',
      maintainer: '',
      license: '',
      creator: '',
      website: '',
      repoURL: '',
      update: '',
      downloads: 0,
      dependencies: {},
      versionFields: [
        { key: 'version',
          label: 'Version',
          class: 'td-meta',
          tdClass: 'list-table-cell'
        },
        { key: 'date',
          label: 'First submitted',
          tdClass: 'list-table-cell',
          formatter: (value) => {
            return moment(value).format('MMMM Do YYYY, HH:mm:ss')
          }
        },
        { key: 'update',
          label: 'Last updated',
          tdClass: 'list-table-cell',
          formatter: (value) => {
            return moment(value).format('MMMM Do YYYY, HH:mm:ss')
          }
        }
      ],
      metaFields: [
        { key: 'metadata',
          label: 'Metadata',
          class: 'td-meta',
          tdClass: 'list-table-cell',
          isRowHeader: true
        },
        {
          key: 'value',
          label: '',
          tdClass: 'metadata-table-cell'
        }
      ],
      miscFields: [
        { key: 'title',
          label: 'Miscellaneous',
          class: 'td-meta',
          tdClass: 'list-table-cell',
          isRowHeader: true
        },
        {
          key: 'value',
          label: '',
          tdClass: 'metadata-table-cell'
        }
      ],
      depFields: [
        { key: 'title',
          label: 'Dependencies',
          class: 'td-meta',
          tdClass: 'list-table-cell',
          isRowHeader: true
        },
        {
          key: 'value',
          label: '',
          tdClass: 'metadata-table-cell'
        }
      ]
    }
  },
  computed: {
  },
  beforeMount () {
    this.getMetadata()
  },
  methods: {
    updateTables () {
      this.$root.$emit('bv::refresh::table', 'version-table')
      this.$root.$emit('bv::refresh::table', 'metadata-table')
      this.$root.$emit('bv::refresh::table', 'misc-table')
      this.$root.$emit('bv::refresh::table', 'dependencies-table')
    },
    getMetadata () {
      let url = '/p/'
      url += this.$route.params.category + '/'
      url += this.$route.params.name + '/'
      url += this.$route.params.version + '/metadata'
      this.$http.get(url).then(
        res => {
          var dataItem = res.body['metadata']
          this.name = dataItem['name']
          this.category = dataItem['category']
          this.version = dataItem['version']
          this.description = dataItem['description']
          this.tags = dataItem['tags']
          this.date = dataItem['created_at']
          this.update = this.date
          this.dependencies = res.body['dependencies']
          this.updateTables()
        },
        err => {
          this.results.error = err.body.error_description
          let route = this.$router.match({ name: 'Error404' })
          this.$router.history.updateRoute(route)
        }
      )
    },
    getTags () {
      return this.tags.split(',')
    },
    getTagsNb () {
      var tags = this.getTags()
      return tags.size()
    },
    detailsProvider (ctx) {
      let items = []
      var newItem = {
        version: this.version,
        date: this.date,
        update: this.update
      }
      items.push(newItem)
      return items || []
    },
    metadataProvider (ctx) {
      let items = []
      var descItem = {
        metadata: 'Description',
        value: this.description
      }
      items.push(descItem)
      var licenseItem = {
        metadata: 'License',
        value: this.license
      }
      items.push(licenseItem)
      var creatorItem = {
        metadata: 'Creator',
        value: this.creator
      }
      items.push(creatorItem)
      var maintainersItem = {
        metadata: 'Maintainer(s)',
        value: this.maintainer
      }
      items.push(maintainersItem)
      return items || []
    },
    miscProvider (ctx) {
      let items = []
      var webItem = {
        title: 'Website',
        value: this.website
      }
      if (this.website) {
        items.push(webItem)
      }
      var repoItem = {
        title: 'Repository',
        value: this.repoURL
      }
      if (this.repoURL) {
        items.push(repoItem)
      }
      var dlItem = {
        title: 'Downloads',
        value: this.downloads
      }
      items.push(dlItem)
      return items || []
    },
    depProvider (ctx) {
      let items = []
      var keys = Object.keys(this.dependencies)
      for (var i = 0; i < keys.length; ++i) {
        var depItem = {
          title: keys[i],
          value: this.dependencies[keys[i]]
        }
        items.push(depItem)
      }
      return items || []
    }
  }
}
</script>

<style scoped>
.top-container {
  margin-top: 150px;
  border-bottom: 1px solid var(--accent);
  padding-bottom: 30px;
}

.mid-container {
  margin-top: 50px;
}

.package-name {
  text-align: center;
  font-size: 45px;
  color: var(--primary-dark);
}

@media (max-width: 360px) {
  .package-name {
    font-size: 25px;
  }
}

.tags-space {
  text-align: center;
  margin-top: 10px;
}

.tag-badge {
  padding: 5px;
  margin-right: 10px;
}
</style>
