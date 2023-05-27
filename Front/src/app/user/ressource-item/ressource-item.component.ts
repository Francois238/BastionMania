import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { RessourceInfo } from '../ressource-info';
import { ActivatedRoute, ParamMap, Router } from '@angular/router';

@Component({
  selector: 'app-ressource-item',
  templateUrl: './ressource-item.component.html',
  styleUrls: ['./ressource-item.component.scss']
})
export class RessourceItemComponent implements OnInit {

  @Input() ressource!: RessourceInfo;
  @Output() newItemEvent = new EventEmitter<string>();

  public bastion_id : string = '';

  constructor(protected router: Router,private activRoute: ActivatedRoute) { }

  ngOnInit(): void {


    this.activRoute.paramMap.subscribe((params: ParamMap) => {
      this.bastion_id = params.get('idBastion') || '';
    });
  }

  getDetail(){

    this.router.navigate([`/user/bastions/${this.bastion_id}/${this.ressource.id}`]);
  }

}
