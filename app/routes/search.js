import Ember from 'ember';

export default Ember.Route.extend({
    queryParams: {
        q: { refreshModel: true },
        page: { refreshModel: true },
    },

    model: function(params) {
        return this.store.find('crate', params);
    },
});
