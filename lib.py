import logging
import graphviz

logger = logging.getLogger(__name__)

def vector_config_to_dot(config: dict) -> str:
    source_components = config.get('sources', {})
    sink_components = config.get('sinks', {})
    transform_components = config.get('transforms', {})
    all_components = {**source_components, **sink_components, **transform_components}

    graph = graphviz.Digraph(graph_attr={
        'nodesep': '0.5',
        # 'rankdir': 'LR',
        'colorscheme': 'prgn4',
    })

    with graph.subgraph(name='cluster_sources') as cluster:
        cluster.attr(color='white')
        _add_nodes('source', source_components, cluster)

    with graph.subgraph(name='cluster_transforms') as cluster:
        cluster.attr(color='white')
        _add_nodes('transform', transform_components, cluster)

    with graph.subgraph(name='cluster_sinks') as cluster:
        cluster.attr(color='white')
        _add_nodes('sink', sink_components, cluster)

    _add_edges(all_components, graph)

    return graph.source

def _get_node_args(kind: str, name: str, spec: dict) -> dict:
    shape_by_kind = {
        'source': 'ellipse',
        'transform': 'box',
        'sink': 'ellipse',
    }
    color_by_kind = {
        'source': '3',
        'transform': '2',
        'sink': '4',
    }
    
    return {
        'label': f"{name}\n({spec['type']})",
        'style': 'rounded, filled',
        'shape': shape_by_kind[kind],
        'color': color_by_kind[kind],
        'colorscheme': 'prgn4',
    }
    
def _add_nodes(component_kind: str, components, graph):
    for component_name, component_spec in components.items():
        graph.node(component_name,
            **_get_node_args(component_kind, component_name, component_spec),
        )
        
def _add_edges(components, graph):
    for component_name, component_spec in components.items():
        input_names = component_spec['inputs'] if 'inputs' in component_spec else []
        for input_name in input_names:
            if input_name.find('.') > -1:
                input_name_parts = input_name.split('.')
                main_input_name = input_name_parts[0]
                subinput_name = ".".join(input_name_parts[1:])
                graph.edge(main_input_name, component_name, label=subinput_name)
            else:
                graph.edge(input_name, component_name)